use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

extern crate serde_json;

use ship::Ship;
use module::{Module, from_primitive};

pub struct Connection {
    index       : usize,
    module      : Module,
    stream      : TcpStream,
    buff_r      : BufReader<TcpStream>,
    pub open    : bool,
}

impl Connection {
    pub fn new(mut stream : TcpStream, ships : &mut Vec<Ship>) -> Connection {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let name = &data[..data.find(":").unwrap()];

        let result = ships.into_iter().position(|ship| ship.name == name);
        let index = match result {
            Some(index) => index,
            None => { 
                let ship = Ship::new(name);
                ships.push(ship);
                ships.len() - 1
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


    pub fn process(&mut self, ships : &mut Vec<Ship>) {
        match self.module {
            Module::Dashboard => {
                let mut send = serde_json::to_string(&ships[self.index]).unwrap();
                send.push_str("\n");
                match self.stream.write(send.as_bytes()) {
                    Ok(_result) => (),
                    Err(_error) => self.open = false,
                }
            }
            Module::Engines => {
                let mut data = String::new();
                match self.buff_r.read_line(&mut data) {
                    Ok(_result) => match data.as_bytes() {
                        b"5\n" => ships[self.index].location.0 += 1,
                        b"0\n" => ships[self.index].location.0 -= 1,
                        b"8\n" => ships[self.index].location.1 += 1,
                        b"2\n" => ships[self.index].location.1 -= 1,
                        b"4\n" => ships[self.index].location.2 += 1,
                        b"6\n" => ships[self.index].location.2 -= 1,
                        _ => (),
                    },
                    Err(_error) => println!("b{}", _error)
                }
            }
            _ => ()
        }
    }
}
