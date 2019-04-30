extern crate space;

use clap::{App, SubCommand};
use std::net::TcpListener;
use std::thread::sleep;
use std::time::{Duration, Instant};

use space::constants;
use space::masses_db::{Init, Masses};
use space::server_connection::ServerConnection;

fn main() {
    let listener = TcpListener::bind(constants::SERVER_IP_PORT).unwrap();
    listener.set_nonblocking(true).unwrap();

    let matches = App::new("space server")
        .subcommand(SubCommand::with_name("--restore"))
        .get_matches();

    let mut masses = match matches.subcommand_name() {
        Some("--restore") => Masses::new(Init::Restore),
        _ => Masses::new(Init::Populate),
    };

    let mut backup_countdown = constants::BACKUP_COUNTDOWN;
    let mut connections: Vec<ServerConnection> = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Some(new_connection) = ServerConnection::new(stream, &mut masses) {
                    let exists = connections.iter().position(|connection| {
                        connection.name == new_connection.name
                            && connection.module_type == new_connection.module_type
                    });
                    if let Some(index) = exists {
                        connections.remove(index);
                    }
                    connections.push(new_connection);
                }
            }
            _ => {
                let timer = Instant::now();

                for connection in connections.iter_mut() {
                    if connection.open {
                        masses.communicate(connection);
                    }
                }

                masses.process();

                if backup_countdown == 0 {
                    masses.backup();
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
