extern crate serde_json;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

use crate::masses::Masses;
use crate::modules::types::ModuleType;

pub struct ServerConnection {
    pub name: String,
    pub module_type: ModuleType,
    pub stream: TcpStream,
    pub buff_r: BufReader<TcpStream>,
    pub open: bool,
}

impl ServerConnection {
    pub fn new(stream: TcpStream, masses: &mut Masses) -> Result<ServerConnection, ()> {
        let mut buff_r = BufReader::new(stream.try_clone().unwrap());
        stream.set_nonblocking(true).unwrap();

        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();

        let data: Vec<&str> = recv.split(':').collect();

        match ServerConnection::check(stream.try_clone().unwrap(), buff_r, masses, data) {
            Ok(connection) => Ok(connection),
            Err(error) => {
                stream
                    .try_clone()
                    .unwrap()
                    .write_all(error.as_bytes())
                    .unwrap();
                Err(())
            }
        }
    }

    pub fn check(
        stream: TcpStream,
        buff_r: BufReader<TcpStream>,
        masses: &mut Masses,
        data: Vec<&str>,
    ) -> Result<ServerConnection, String> {
        if data.len() != 4 {
            return Err(String::from("Input data is false."));
        }

        masses.validate(
            data[0].to_string(),
            data[1].to_string(),
            data[2].to_string(),
        )?;

        let module_type: Result<ModuleType, serde_json::Error> =
            serde_json::from_str(&data[3].replace("\n", ""));

        if module_type.is_err() {
            return Err(String::from("Module doesn't exist."));
        }

        Ok(ServerConnection {
            name: String::from(data[1]),
            module_type: module_type.unwrap(),
            stream,
            buff_r,
            open: true,
        })
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
