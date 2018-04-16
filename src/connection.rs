extern crate serde_json;

use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

use module::{Module, ModuleType};
use mass::{Mass, MassType};

pub struct Connection {
    pub name        : String,
    pub module      : Module,
    pub stream      : TcpStream,
    pub buff_r      : BufReader<TcpStream>,
    pub open        : bool,
}

impl Connection {
    pub fn new(mut stream : TcpStream, masses : &mut HashMap<String, Mass>) -> Connection {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let name = &recv[..recv.find(":").unwrap()];

        let ship = masses.entry(name.to_string()).or_insert(Mass::new_ship());

        let send = match ship.mass_type {
            MassType::Ship{ref modules, ..} => serde_json::to_string(modules).unwrap() + "\n",
            _ => String::new(),
        };
        stream.write(send.as_bytes()).unwrap();

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let module : Module = serde_json::from_str(&recv.replace("\n","")).unwrap();

        stream.set_nonblocking(true).unwrap();
        Connection { 
            name        : String::from(name),
            module      : module,
            stream      : stream,
            buff_r      : buff_r,
            open        : true,
        }
    }

    pub fn process(&mut self, mut masses : &mut HashMap<String, Mass>) {
        self.open = match self.module.module_type {
            ModuleType::Dashboard => self.server_dashboard(&mut masses),
            ModuleType::Engines => self.server_engines(&mut masses),
            ModuleType::Mining{..} => self.server_mining(&mut masses),
            ModuleType::Navigation{..} => self.server_navigation(&mut masses),
        };
    }
}
