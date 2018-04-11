use std::thread::sleep;
use std::time::Duration;
use std::net::TcpListener;
use std::collections::HashMap;

extern crate space;
use space::mass::Mass;
use space::astroid::Astroid;
use space::connection::Connection;


fn populate() -> HashMap<String, Box<Mass>> {
    let mut masses : HashMap<String, Box<Mass>> = HashMap::new();

    for _ in 0..10 {
        masses.insert("zz".to_string(), Box::new(Astroid::new()));
    }

    masses
}

fn main() {
    let listener = TcpListener::bind("localhost:6000").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut masses = populate();

    let mut connections = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => connections.push(Connection::new(stream, &mut masses)),
            _ => {
                for i in 0..connections.len() {
                    connections[i].process(&mut masses);
                }
                connections.retain(|connection| connection.open);

                for mass in masses.values_mut() {
                    mass.process();
                }

                sleep(Duration::from_millis(100));
            }
        }
    }
}
