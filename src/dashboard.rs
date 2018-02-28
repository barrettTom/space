use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;
use std::io::Write;

extern crate serde_json;

use ship::Ship;
use mass::Mass;

pub fn client_dashboard(mut buff_r : BufReader<TcpStream>) {
    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let ship : Ship = serde_json::from_str(&data).unwrap();
        println!("{:?}", ship);
    }
}

pub fn server_dashboard(mut ship_string : String, mut stream : &TcpStream) -> bool {
    ship_string.push_str("\n");
    match stream.write(ship_string.as_bytes()) {
        Ok(_result) => true,
        Err(_error) => false,
    }
}
