extern crate space;

use legion::prelude::*;
use std::net::TcpListener;
use std::thread::{sleep};
use std::time::{Duration, Instant};

use space::constants;
use space::server_connection::ServerConnection;

fn populate(world: &mut World) {
    world.insert(
        (Astroid, true),
        vec![(Velocity::default(), Position::default(), Storage::default())],
    );
}

    /*
fn backup(_masses: HashMap<String, Mass>) {
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
    */

#[derive(Debug, Clone, Copy, PartialEq)]
struct Astroid;

#[derive(Debug, Clone, Default, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum ItemType {
    CrudeMinerals,
    //    Iron,
    //    Hydrogen,
}

impl Default for ItemType {
    fn default() -> ItemType {
        ItemType::CrudeMinerals
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Item {
    item_type: ItemType,
    name: String,
    size: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Storage {
    items: Vec<Item>,
    carrying: usize,
    capactiy: usize,
}

fn main() {
    let listener = TcpListener::bind("localhost:6000").unwrap();
    listener.set_nonblocking(true).unwrap();

    /*
    let matches = App::new("space server")
        .subcommand(SubCommand::with_name("--restore"))
        .get_matches();
    */

    let universe = Universe::new();
    let mut world = universe.create_world();

    populate(&mut world);

    //let mut masses = match matches.subcommand_name() {
    //    Some("--restore") => restore(),
    //    _ => populate(),
    //};

    //let mut backup_countdown = constants::BACKUP_COUNTDOWN;
    let mut connections: Vec<ServerConnection> = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                /*
                let new_connection = ServerConnection::new(stream, &mut masses);
                let exists = connections.iter().position(|connection| {
                    connection.name == new_connection.name
                        && connection.module_type == new_connection.module_type
                });
                if let Some(index) = exists {
                    connections.remove(index);
                }
                connections.push(new_connection);
                */
            }
            _ => {
                let timer = Instant::now();

                for connection in &mut connections {
                    if connection.open {
                        /*
                        let mut ship = masses.remove(&connection.name).unwrap();

                        let send = ship.get_client_data(connection.module_type.clone(), &masses);
                        connection.open = connection.stream.write(send.as_bytes()).is_ok();

                        let recv = connection.receive();
                        ship.give_received_data(connection.module_type.clone(), recv);

                        masses.insert(connection.name.clone(), ship);
                        */
                    }
                }

                /*
                for key in masses.clone().keys() {
                    let mut mass = masses.remove(key).unwrap();
                    mass.process(&mut masses);
                    masses.insert(key.to_string(), mass);
                }
                */

                /*
                if backup_countdown == 0 {
                    let masses_clone = masses.clone();
                    spawn(move || backup(masses_clone));
                    backup_countdown = constants::BACKUP_COUNTDOWN;
                }
                */

                if timer.elapsed().as_millis() < constants::LOOP_DURATION_MS.into() {
                    sleep(Duration::from_millis(
                        constants::LOOP_DURATION_MS - timer.elapsed().as_millis() as u64,
                    ));
                }
                //backup_countdown -= 1;
            }
        }
    }
}
