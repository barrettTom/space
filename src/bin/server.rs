use std::thread::sleep;
use std::time::Duration;
use std::net::TcpListener;

extern crate space;
use space::mass::Mass;
use space::astroid::Astroid;
use space::connection::Connection;


fn populate() -> Vec<Box<Mass>> {
    let mut masses : Vec<Box<Mass>> = Vec::new();

    masses.push(Box::new(Astroid::new("cZfAJ", (10.0, -5.0, 4.0))));

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

                sleep(Duration::from_millis(100));
            }
        }
    }
}
