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
    pub fn new(
        mut stream: TcpStream,
        masses: &mut HashMap<String, Mass>,
    ) -> Option<ServerConnection> {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let name = &recv[..recv.find(':').unwrap()];

        match masses.get(&name.to_string()) {
            Some(ship) => {
                let send = serde_json::to_string(&ship.get_modules()).unwrap() + "\n";
                stream.write_all(send.as_bytes()).unwrap();

                let mut recv = String::new();
                buff_r.read_line(&mut recv).unwrap();
                let module_type: ModuleType =
                    serde_json::from_str(&recv.replace("\n", "")).unwrap();

                stream.set_nonblocking(true).unwrap();
                Some(ServerConnection {
                    name: String::from(name),
                    module_type,
                    stream,
                    buff_r,
                    open: true,
                })
            }
            None => None,
        }
    }

    pub fn receive(&mut self) -> String {
        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(result) => {
                if result == 0 {
                    self.open = false;
                    String::new()
                } else {
                    recv.replace("\n", "")
                }
            }
            Err(_) => String::new(),
        }
    }
}
