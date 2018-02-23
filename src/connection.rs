use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

extern crate serde_json;

use ship::Ship;
use mass::Mass;
use module::{Module, from_primitive};

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
                let ship = Box::new(Ship::new(name, (0,0,0)));
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
                let mut location = masses[self.index].location();
                let mut data = String::new();
                match self.buff_r.read_line(&mut data) {
                    Ok(result) => match data.as_bytes() {
                        b"5\n" => location.0 += 1,
                        b"0\n" => location.0 -= 1,
                        b"8\n" => location.1 += 1,
                        b"2\n" => location.1 -= 1,
                        b"4\n" => location.2 += 1,
                        b"6\n" => location.2 -= 1,
                        _ => {
                            if result == 0 {
                                self.open = false;
                            }
                        },
                    },
                    Err(_error) => (),
                }
                masses[self.index].set_location(location);
            }
            Module::Navigation => {
                let ship = &masses[self.index].downcast_ref::<Ship>().unwrap();

                let mut within_range = Vec::new();
                for mass in masses.iter() {
                    if distance(ship.location(), mass.location()) > ship.range() {
                        within_range.push(mass);
                    }
                }
                let mut send = String::new();
                for mass in within_range {
                    send.push_str(&mass.serialize());
                    send.push_str("\n");
                }
                match self.stream.write(send.as_bytes()) {
                    Ok(_result) => (),
                    Err(_error) => self.open = false,
                }
            }
        }
    }

}

fn distance(l0 : (isize, isize, isize), l1 : (isize, isize, isize)) -> f64 {
    (((l1.0-l0.0).pow(2) + (l1.1-l0.1).pow(2) + (l1.2-l0.2).pow(2)) as f64).sqrt()
}
