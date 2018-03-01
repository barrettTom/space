use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

extern crate serde_json;

use ship::Ship;
use mass::Mass;
use engines::server_engines;
use dashboard::server_dashboard;
use navigation::server_navigation;
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

    pub fn process(&mut self, mut masses : &mut Vec<Box<Mass>>) {
        self.open = match self.module {
            Module::Dashboard => server_dashboard(masses[self.index].serialize(), &self.stream),
            Module::Engines => server_engines(&mut masses[self.index], &mut self.buff_r),
            Module::Navigation => server_navigation(&mut masses.to_vec(), &mut masses[self.index], &self.stream, &mut self.buff_r),
        };
    }
}
