extern crate serde_json;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

use crate::mass::Mass;
use crate::modules::types::ModuleType;

pub struct ServerConnection {
    pub name: String,
    pub module_type: ModuleType,
    pub stream: TcpStream,
    pub buff_r: BufReader<TcpStream>,
    pub open: bool,
}

impl ServerConnection {
    pub fn new(mut stream: TcpStream, masses: &mut HashMap<String, Mass>) -> ServerConnection {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let name = &recv[..recv.find(':').unwrap()];

        let ship = masses
            .entry(name.to_string())
            .or_insert_with(Mass::new_ship);

        let send = serde_json::to_string(&ship.get_modules()).unwrap() + "\n";
        stream.write_all(send.as_bytes()).unwrap();

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let module_type: ModuleType = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        stream.set_nonblocking(true).unwrap();
        ServerConnection {
            name: String::from(name),
            module_type,
            stream,
            buff_r,
            open: true,
        }
    }

    pub fn process(&mut self, mut masses: &mut HashMap<String, Mass>) {
        match self.module_type {
            ModuleType::Mining => self.server_mining(&mut masses),
            ModuleType::Engines => self.server_engines(&mut masses),
            ModuleType::Refinery => self.server_refinery(&mut masses),
            ModuleType::Dashboard => self.server_dashboard(&mut masses),
            ModuleType::Navigation => self.server_navigation(&mut masses),
            ModuleType::Tractorbeam => self.server_tractorbeam(&mut masses),
            ModuleType::Construction => self.server_construction(&mut masses),
        }
    }
}
