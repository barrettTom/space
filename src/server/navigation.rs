extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use mass::{Mass, MassType};
use math::distance;
use connection::Connection;

impl Connection {
    pub fn server_navigation(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let masses_clone = masses.clone();
        let ship = masses.get_mut(&self.name).unwrap();
        let ship_position = ship.position;

        match ship.mass_type {
            MassType::Ship{ref mut targeting, ..} => {
                let target_name = targeting.clone().target;
                match target_name {
                    Some(target_name) => {
                        let target = masses_clone.get(&target_name).unwrap();
                        if distance(target.position, ship.position) > targeting.range {
                            targeting.target = None;
                        }
                    },
                    _ => (),
                }

                let within_range : HashMap<&String, &Mass> = masses_clone.iter().filter(|&(_, mass)|
                                                                                        distance(ship_position, mass.position) < targeting.range)
                                                                                        .collect();

                let send = serde_json::to_string(&within_range).unwrap() + "\n";
                match self.stream.write(send.as_bytes()) {
                    Ok(_result) => (),
                    Err(_error) => return false,
                }

                let mut recv = String::new();
                match self.buff_r.read_line(&mut recv) {
                    Ok(_result) => (),
                    Err(_error) => (),
                }
                if !recv.is_empty() {
                    targeting.give_target(Some(recv.replace("\n", "")));
                }
            },
            _ => (),
        }

        true
    }
}
