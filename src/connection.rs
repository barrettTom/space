use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::collections::HashMap;

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
    pub fn new(mut stream : TcpStream, masses : &mut HashMap<String, Box<Mass>>) -> Connection {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let name = &recv[..recv.find(":").unwrap()];

        let mass = masses.entry(name.to_string()).or_insert(Box::new(Ship::new(name, (0.0, 0.0, 0.0))));
        let ship = mass.downcast_ref::<Ship>().unwrap();

        let modules = ship.recv_modules();
        stream.write(modules.as_bytes()).unwrap();

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let module : Module = serde_json::from_str(&recv.replace("\n","")).unwrap();

        stream.set_nonblocking(true).unwrap();

        Connection { 
            name    : String::from(name),
            module  : module,
            stream  : stream,
            buff_r  : buff_r,
            open    : true,
        }
    }

    pub fn process(&mut self, mut masses : &mut HashMap<String, Box<Mass>>) {
        self.open = match self.module {
            Module::Engines => self.server_engines(&mut masses),
            Module::Dashboard => self.server_dashboard(&mut masses),
            Module::Navigation => self.server_navigation(&mut masses),
            Module::Mining => self.server_mining(&mut masses),
        };
    }
}
