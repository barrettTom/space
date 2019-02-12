extern crate space;

use std::collections::HashMap;
use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;

use space::constants;
use space::mass::Mass;
use space::math::rand_name;
use space::server::connection::ServerConnection;

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
                    connection.process(&mut masses);
                }

                for mass in masses.values_mut() {
                    mass.process();
                }

                sleep(Duration::from_millis(constants::SLEEP_DURATION));
            }
        }
    }
}
