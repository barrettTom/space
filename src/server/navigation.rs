extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::time::SystemTime;
use std::collections::HashMap;

use math::distance;
use module::{ModuleType, NavigationStatus};
use mass::{Mass, MassType};
use connection::Connection;

impl Connection {
    pub fn server_navigation(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let masses_clone = masses.clone();
        let ship = masses.get_mut(&self.name).unwrap();
        let ship_position = ship.position;

        match ship.mass_type {
            MassType::Ship{ref mut modules, ..} => {
                match modules.get_mut("Navigation").unwrap().module_type {
                    ModuleType::Navigation{ref mut target_name, ref mut start, ref mut status, ref range, ..} => {
                        match target_name.clone() {
                            Some(name) => {
                                let target = masses_clone.get(&name).unwrap();
                                if distance(target.position, ship.position) > *range {
                                    *target_name = None;
                                }
                            },
                            _ => (),
                        }

                        let within_range : HashMap<&String, &Mass> = masses_clone.iter().filter(|&(_, mass)|
                                                                                                distance(ship_position, mass.position) < *range)
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
                            *target_name = Some(recv.replace("\n", ""));
                            *start = Some(SystemTime::now());
                            *status = NavigationStatus::Targeting;
                        }
                    },
                _ => (),
                }
            }
        _ => (),
        }
        true
    }
}
