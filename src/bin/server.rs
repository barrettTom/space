use std::thread::sleep;
use std::time::Duration;
use std::net::TcpListener;

extern crate space;
use space::connection::Connection;

fn main() {
    let listener = TcpListener::bind("localhost:6000").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut ships = Vec::new();

    let mut connections = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => connections.push(Connection::new(stream, &mut ships)),
            _ => {
                for i in 0..connections.len() {
                    connections[i].process(&mut ships);
                }
                connections.retain(|connection| connection.open );

                sleep(Duration::from_millis(100));
            }
        }
    }
}
