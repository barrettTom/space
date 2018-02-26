use std::net::TcpStream;
use std::io::BufRead;
use std::io::BufReader;

extern crate erased_serde;
extern crate serde_json;

use std::collections::BTreeMap;

use mass::Mass;
use ship::Ship;
use astroid::Astroid;

use erased_serde::Deserializer;

pub fn Navigation(mut stream :TcpStream, mut buff_r : BufReader<TcpStream>){
    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();

        let string_masses = data.split(";");
        let mut masses : Vec<Box<Mass>> = Vec::new();
        for string_mass in string_masses {
            if string_mass.len() == 1 {
                break;
            }
            let json = &mut serde_json::de::Deserializer::from_slice(string_mass.as_bytes());
            let mut deserialized : Box<Deserializer> = Box::new(Deserializer::erase(json));

            if string_mass.contains("Ship") {
                let mass : Ship = erased_serde::deserialize(&mut deserialized).unwrap();
                masses.push(Box::new(mass));
            }
            else {
                let mass : Astroid = erased_serde::deserialize(&mut deserialized).unwrap();
                masses.push(Box::new(mass));
            }
        }
    }
}
