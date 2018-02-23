use std::net::TcpStream;
use std::io::BufRead;
use std::io::BufReader;

extern crate serde_json;

use mass::Mass;
use ship::Ship;

pub fn Navigation(mut stream :TcpStream, mut buff_r : BufReader<TcpStream>){
    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let string_masses = data.split(",");
        //let masses : Vec<Box<Mass>> = Vec::new();
        for string_mass in string_masses {
            //let mass = Box::new(Ship::new("",(0,0,0)));
            //mass.deserialize(string_mass);

            //let mass : Box<Mass> = serde_json::from_str(string_mass).unwrap();
            //masses.push(Box::new(mass));
        }
    }
}
