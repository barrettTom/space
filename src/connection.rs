use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

extern crate serde_json;

use ship::Ship;
use mass::Mass;
use module::Module;

pub struct Connection {
    pub index   : usize,
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

        let result = masses.into_iter().position(|ship| ship.name() == name);
        let index = match result {
            Some(index) => index,
            None => { 
                let ship = Box::new(Ship::new(name, (0.0, 0.0, 0.0)));
                masses.push(ship);
                masses.len() - 1
            },
        };

        let modules = masses[index].downcast_ref::<Ship>().unwrap().get_modules();
        stream.write(modules.as_bytes()).unwrap();

        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
        let module : Module = serde_json::from_str(&data.replace("\n","")).unwrap();

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
            Module::Engines => self.server_engines(&mut masses),
            Module::Dashboard => self.server_dashboard(&mut masses),
            Module::Navigation => self.server_navigation(&mut masses),
        };
    }
}
