extern crate space;

use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;

use space::constants;
use space::mass::Mass;
use space::math::rand_name;
use space::server_connection::ServerConnection;

fn populate() -> HashMap<String, Mass> {
    let mut masses: HashMap<String, Mass> = HashMap::new();

    for _ in 0..constants::ASTROID_COUNT {
        masses.insert(rand_name(), Mass::new_astroid());
    }

    masses
}

fn main() {
    let listener = TcpListener::bind("localhost:6000").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut masses = populate();

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
                for connection in &mut connections {
                    if connection.open {
                        let mut ship = masses.remove(&connection.name).unwrap();

                        let send = ship.get_client_data(connection.module_type.clone(), &masses);
                        connection.open = connection.stream.write(send.as_bytes()).is_ok();

                        let recv = connection.receive();
                        ship.give_received_data(connection.module_type.clone(), recv, &masses);

                        masses.insert(connection.name.clone(), ship);
                    }
                }

                for key in masses.clone().keys() {
                    let mut mass = masses.remove(key).unwrap();
                    mass.process(&mut masses);
                    masses.insert(key.to_string(), mass);
                }

                sleep(Duration::from_millis(constants::SLEEP_DURATION));
            }
        }
    }
}
