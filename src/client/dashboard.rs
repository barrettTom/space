extern crate serde_json;

use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

use mass::Mass;

pub fn client_dashboard(mut buff_r : BufReader<TcpStream>) {
    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let ship : Mass = serde_json::from_str(&recv).unwrap();
        println!("{:?}", ship);
    }
}
