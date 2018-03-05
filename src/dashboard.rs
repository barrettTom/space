use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;
use std::io::Write;

extern crate serde_json;

use mass::Mass;
use ship::Ship;
use connection::Connection;

pub fn client_dashboard(mut buff_r : BufReader<TcpStream>) {
    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let ship : Ship = serde_json::from_str(&data).unwrap();
        println!("{:?}", ship);
    }
}

impl Connection {
    pub fn server_dashboard(&mut self, masses : &mut Vec<Box<Mass>>) -> bool {
        let mut ship_string = masses[self.index].serialize();
        ship_string.push_str("\n");
        match self.stream.write(ship_string.as_bytes()) {
            Ok(_result) => true,
            Err(_error) => false,
        }
    }
}
