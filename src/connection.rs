use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

extern crate serde_json;

use ship::Ship;
use module::{Module, from_primitive};

pub struct Connection {
    name        : String,
    ship        : Ship,
    module      : Module,
    stream      : TcpStream,
    buff_r      : BufReader<TcpStream>,
    pub open    : bool,
}

impl Connection {
    pub fn new(mut stream : TcpStream) -> Connection {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut data = String::new();
        buff_r.read_line(&mut data);
        let name = &data[..data.find(":").unwrap()];

        let modules = b"dashboard,navigation,engine\n";
        stream.write(modules);

        let mut data = String::new();
        buff_r.read_line(&mut data);
        let num = data.replace("\n", "");
        let module = from_primitive(num.parse::<isize>().unwrap());

        Connection { 
            name    : String::from(name),
            ship    : Ship::new(),
            module  : module,
            stream  : stream,
            buff_r  : buff_r,
            open    : true,
        }
    }

    pub fn process(&mut self) {
        match self.module {
            Module::Dashboard => {
                let mut send = serde_json::to_string(&self.ship).unwrap();
                send.push_str("\n");
                self.stream.write(send.as_bytes());
            }
            _ => ()
        }
    }
}

