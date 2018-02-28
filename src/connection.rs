use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

extern crate serde_json;
extern crate erased_serde;

use erased_serde::Deserializer;

use ship::Ship;
use mass::Mass;
use astroid::Astroid;
use module::{Module, from_primitive};
use math::distance;

pub struct Connection {
    index       : usize,
    module      : Module,
    stream      : TcpStream,
    buff_r      : BufReader<TcpStream>,
    pub open    : bool,
}

impl Connection {
    pub fn new(mut stream : TcpStream, masses : &mut Vec<Box<Mass>>) -> Connection {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let name = &data[..data.find(":").unwrap()];

        let result = masses.into_iter().position(|ship| ship.name() == name);
        let index = match result {
            Some(index) => index,
            None => { 
                let ship = Box::new(Ship::new(name, (0.0,0.0,0.0)));
                masses.push(ship);
                masses.len() - 1
            },
        };

        let modules = b"dashboard,navigation,engine\n";
        stream.write(modules).unwrap();

        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let module = from_primitive(data);

        stream.set_nonblocking(true).unwrap();

        Connection { 
            index   : index,
            module  : module,
            stream  : stream,
            buff_r  : buff_r,
            open    : true,
        }
    }

    pub fn process(&mut self, masses : &mut Vec<Box<Mass>>) {
        match self.module {
            Module::Dashboard => {
                let mut send = masses[self.index].serialize();
                send.push_str("\n");
                match self.stream.write(send.as_bytes()) {
                    Ok(_result) => (),
                    Err(_error) => self.open = false,
                }
            }
            Module::Engines => {
                let mut acceleration = (0.0, 0.0, 0.0);
                let mut data = String::new();
                match self.buff_r.read_line(&mut data) {
                    Ok(result) => match data.as_bytes() {
                        b"5\n" => acceleration.0 += 0.1,
                        b"0\n" => acceleration.0 -= 0.1,
                        b"8\n" => acceleration.1 += 0.1,
                        b"2\n" => acceleration.1 -= 0.1,
                        b"4\n" => acceleration.2 += 0.1,
                        b"6\n" => acceleration.2 -= 0.1,
                        b"-\n" => masses[self.index].slow(),
                        _ => {
                            if result == 0 {
                                self.open = false;
                            }
                        },
                    },
                    Err(_error) => (),
                }
                masses[self.index].give_acceleration(acceleration);
            }
            Module::Navigation => {
                let ship = &masses[self.index].downcast_ref::<Ship>().unwrap();

                let within_range : Vec<&Box<Mass>> = masses.iter().filter(|mass|
                distance(ship.position(), mass.position()) < ship.range()).collect();

                let mut send = String::new();
                for mass in within_range {
                    send.push_str(&mass.serialize());
                    send.push_str(";");
                }
                send.push_str("\n");
                match self.stream.write(send.as_bytes()) {
                    Ok(_result) => (),
                    Err(_error) => self.open = false,
                }

                let mut string_mass = String::new();
                self.buff_r.read_line(&mut string_mass).unwrap();
                if string_mass.len() > 0 {
                    let json = &mut serde_json::de::Deserializer::from_slice(string_mass.as_bytes());
                    let mut deserialized : Box<Deserializer> = Box::new(Deserializer::erase(json));
                    if string_mass.contains("Ship") {
                        let mass : Ship = erased_serde::deserialize(&mut deserialized).unwrap();
                    }
                    else {
                        let mass : Astroid = erased_serde::deserialize(&mut deserialized).unwrap();
                    }
                }
            }
        }
    }
}
