use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;

extern crate serde_json;

use ship::Ship;

pub fn Dashboard(mut buff_r : BufReader<TcpStream>) {
    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let ship : Ship = serde_json::from_str(&data).unwrap();
        println!("Location: ({},{},{})",    ship.location.0,
                                            ship.location.1,
                                            ship.location.2);
    }
}
