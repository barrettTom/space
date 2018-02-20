use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

extern crate space;
use space::connection::Connection;

fn main() {
    let listener = TcpListener::bind("localhost:6000").unwrap();
    listener.set_nonblocking(true);

    let mut connections = Vec::new();
    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => connections.push(Connection::new(stream)),
                _ => (),
            }
            for i in 0..connections.len() {
                connections[i].process();
            }
            connections.retain(|connection| connection.open );
        }
    }
}
