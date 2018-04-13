extern crate serde_json;

use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

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
    pub fn server_mining(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let masses_clone = masses.clone();
        let ship = masses.get_mut(&self.name).unwrap();

        let (mining, targeting) = match ship.mass_type {
            MassType::Ship{ref targeting, ref mut mining, ..} => (Some(mining), Some(targeting.clone())),
            _ => (None, None),
        };
        let mining = mining.unwrap();

        let target = masses_clone.get(&targeting.unwrap().target.unwrap());
        let has_astroid_target = match target {
            Some(target) => match target.mass_type {
                MassType::Astroid{..} => true,
                _ => false,
            },
            None => false,
        };

        let is_within_range = match has_astroid_target {
            true => match target {
                Some(target) => mining.range > distance(ship.position, target.position),
                _ => false,
            }
            _ => false,
        };

        let send = serde_json::to_string(&ServerData {
                                            has_astroid_target  : has_astroid_target,
                                            is_within_range     : is_within_range,
                                            mining_range        : mining.range,
                                            mining_status       : mining.status,
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
                        match mining.status {
                            true => mining.stop(),
                            false => mining.start(),
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
