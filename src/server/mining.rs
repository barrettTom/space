extern crate serde_json;

use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

use ship::Ship;
use math::distance;
use mass::{Mass, MassType};
use connection::Connection;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ServerData {
    has_astroid_target  : bool,
    is_within_range     : bool,
    mining_range        : f64,
    mining_status       : bool,
}

impl Connection {
    pub fn server_mining(&mut self, masses : &mut HashMap<String, Box<Mass>>) -> bool {
        let masses_clone = masses.clone();
        let mass = masses.get_mut(&self.name).unwrap();
        let ship = mass.downcast_mut::<Ship>().unwrap();

        let target = match ship.recv_target() {
            Some(name) => masses_clone.get(&name),
            None => None,
        };

        let has_astroid_target = match target {
            Some(target) => match target.recv_mass_type() {
                MassType::Ship => false,
                MassType::Astroid => true,
            },
            None => false,
        };

        let is_within_range = match has_astroid_target {
            true => match target {
                Some(target) => match ship.recv_mining_range() > distance(ship.position(), target.position()) {
                    true => true,
                    false => false,
                },
                None => false,
            }
            false => false,
        };

        let send = serde_json::to_string(&ServerData {
                                            has_astroid_target  : has_astroid_target,
                                            is_within_range     : is_within_range,
                                            mining_range        : ship.recv_mining_range(),
                                            mining_status       : ship.recv_mining_status(),
                                         }).unwrap() + "\n";

        match self.stream.write(send.as_bytes()) {
            Ok(_result) => (),
            Err(_error) => return false,
        }

        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(result) => match recv.as_bytes() {
                b"F\n" => {
                    if is_within_range {
                        match ship.recv_mining_status() {
                            true => ship.stop_mining(),
                            false => ship.start_mining(),
                        }
                    }
                },
                _ => {
                    if result == 0 {
                        return false
                    }
                },
            }
            Err(_error) => (),
        }

        true
    }
}
