extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use mass::Mass;
use ship::Ship;
use math::distance;
use connection::Connection;

impl Connection {
    pub fn server_navigation(&mut self, masses : &mut HashMap<String, Box<Mass>>) -> bool {
        let masses_clone = masses.clone();
        let mass = masses.get_mut(&self.name).unwrap();
        let ship = mass.downcast_mut::<Ship>().unwrap();

        match ship.recv_target() {
            Some(name) => {
                let target = masses_clone.get(&name).unwrap();
                if distance(target.position(), ship.position()) > ship.recv_range() {
                    ship.give_target(None);
                }
            }
            None => (),
        }

        let within_range : HashMap<&String, &Box<Mass>> = masses_clone.iter().filter(|&(_, mass)|
                                                                                     distance(ship.position(), mass.position()) < ship.recv_range())
                                                                                     .collect();
        let mut send = String::new();
        for (name, mass) in within_range {
            send.push_str(name);
            send.push_str("@");
            send.push_str(&mass.serialize());
            send.push_str(";");
        }
        send.push_str("\n");
        match self.stream.write(send.as_bytes()) {
            Ok(_result) => (),
            Err(_error) => return false,
        }

        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(_result) => (),
            Err(_error) => (),
        }
        if recv.len() > 0 {
            ship.give_target(Some(recv.replace("\n", "")));
        }
        true
    }
}
