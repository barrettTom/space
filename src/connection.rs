use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

extern crate serde_json;

use ship::Ship;
use mass::Mass;
use module::Module;

pub struct Connection {
    pub name    : String,
    pub module  : Module,
    pub stream  : TcpStream,
    pub buff_r  : BufReader<TcpStream>,
    pub open    : bool,
}

impl Connection {
    pub fn new(mut stream : TcpStream, masses : &mut Vec<Box<Mass>>) -> Connection {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let name = &data[..data.find(":").unwrap()];

        match masses.iter().find(|ship| ship.name() == name).is_some() {
            false => masses.push(Box::new(Ship::new(name, (0.0, 0.0, 0.0)))),
            _ => (),
        }

        let mass = masses.iter().find(|ship| ship.name() == name).unwrap();
        let ship = mass.downcast_ref::<Ship>().unwrap();

        let modules = ship.get_modules();
        stream.write(modules.as_bytes()).unwrap();

        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let module : Module = serde_json::from_str(&data.replace("\n","")).unwrap();

        stream.set_nonblocking(true).unwrap();

        Connection { 
            name    : String::from(name),
            module  : module,
            stream  : stream,
            buff_r  : buff_r,
            open    : true,
        }
    }

    pub fn process(&mut self, mut masses : &mut Vec<Box<Mass>>) {
        self.open = match self.module {
            Module::Engines => self.server_engines(&mut masses),
            Module::Dashboard => self.server_dashboard(&mut masses),
            Module::Navigation => self.server_navigation(&mut masses),
        };
    }
}
