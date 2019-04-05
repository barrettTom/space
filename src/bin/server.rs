extern crate space;

use clap::{App, SubCommand};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant, SystemTime};

use space::constants;
use space::mass::Mass;
use space::math::{get_db_url, rand_name};
use space::models::MassEntry;
use space::schema::masses::dsl::masses as db_masses;
use space::schema::masses::dsl::name as name_column;
use space::server_connection::ServerConnection;

fn populate() -> HashMap<String, Mass> {
    let mut masses: HashMap<String, Mass> = HashMap::new();

    for _ in 0..constants::ASTROID_COUNT {
        masses.insert(rand_name(), Mass::new_astroid());
    }

    masses
}

fn backup(masses: HashMap<String, Mass>) {
    let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");
    let timestamp = SystemTime::now();
    for (name, mass) in masses {
        let mass_entry = mass.to_mass_entry(name.to_string(), timestamp);
        diesel::insert_into(db_masses)
            .values(&mass_entry)
            .on_conflict(name_column)
            .do_update()
            .set(&mass_entry)
            .execute(&connection)
            .expect("Cannot backup");
    }
}

fn restore() -> HashMap<String, Mass> {
    let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");
    db_masses
        .load::<MassEntry>(&connection)
        .expect("Cannot query, are you sure you can restore?")
        .iter()
        .map(|mass_entry| mass_entry.to_mass())
        .collect()
}

fn main() {
    let listener = TcpListener::bind("localhost:6000").unwrap();
    listener.set_nonblocking(true).unwrap();

    let matches = App::new("space server")
        .subcommand(SubCommand::with_name("--restore"))
        .get_matches();

    let mut masses = match matches.subcommand_name() {
        Some("--restore") => restore(),
        _ => populate(),
    };

    let mut backup_countdown = constants::BACKUP_COUNTDOWN;
    let mut connections: Vec<ServerConnection> = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let new_connection = ServerConnection::new(stream, &mut masses);
                let exists = connections.iter().position(|connection| {
                    connection.name == new_connection.name
                        && connection.module_type == new_connection.module_type
                });
                if let Some(index) = exists {
                    connections.remove(index);
                }
                connections.push(new_connection);
            }
            _ => {
                let timer = Instant::now();

                for connection in &mut connections {
                    if connection.open {
                        let mut ship = masses.remove(&connection.name).unwrap();

                        let send = ship.get_client_data(connection.module_type.clone(), &masses);
                        connection.open = connection.stream.write(send.as_bytes()).is_ok();

                        let recv = connection.receive();
                        ship.give_received_data(connection.module_type.clone(), recv);

                        masses.insert(connection.name.clone(), ship);
                    }
                }

                for key in masses.clone().keys() {
                    let mut mass = masses.remove(key).unwrap();
                    mass.process(&mut masses);
                    masses.insert(key.to_string(), mass);
                }

                if backup_countdown == 0 {
                    let masses_clone = masses.clone();
                    spawn(move || backup(masses_clone));
                    backup_countdown = constants::BACKUP_COUNTDOWN;
                }

                if timer.elapsed().as_millis() < constants::LOOP_DURATION_MS.into() {
                    sleep(Duration::from_millis(
                        constants::LOOP_DURATION_MS - timer.elapsed().as_millis() as u64,
                    ));
                }
                backup_countdown -= 1;
            }
        }
    }
}
