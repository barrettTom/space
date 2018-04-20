extern crate serde_json;

use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

use math::distance;
use mass::{Mass, MassType};
use module::{Module, ModuleType};
use server::connection::ServerConnection;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiningData {
    pub has_astroid_target  : bool,
    pub is_within_range     : bool,
    pub range               : f64,
    pub status              : bool,
}

impl ServerConnection {
    pub fn server_mining(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let masses_clone = masses.clone();
        let ship = masses.get_mut(&self.name).unwrap();
        let ship_clone = ship.clone();


        if let MassType::Ship{ref mut modules, ..} = ship.mass_type {
            let mining_data = get_mining_data(ship_clone, modules, masses_clone);

            let send = serde_json::to_string(&mining_data).unwrap() + "\n";
            match self.stream.write(send.as_bytes()) {
                Ok(_result) => (),
                Err(_error) => return false,
            }

            if let ModuleType::Mining{ref mut status, ..} = modules.get_mut("Mining").unwrap().module_type {
                let mut recv = String::new();
                match self.buff_r.read_line(&mut recv) {
                    Ok(result) => match recv.as_bytes() {
                        b"F\n" => {
                            if mining_data.is_within_range {
                                *status = !*status;
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
            }
        }
        true
    }
}

fn get_mining_data(ship : Mass, modules : &mut HashMap<String, Module>, masses_clone : HashMap<String, Mass>) -> MiningData {
    let mut mining_range = 0.0;
    let mut mining_status = false;
    if let ModuleType::Mining{ref range, ref status, ..} = modules.get("Mining").unwrap().module_type {
        mining_range = *range;
        mining_status = *status;
    }

    let mut has_astroid_target = false;
    let mut is_within_range = false;
    if let ModuleType::Navigation{ref target_name, ..} = modules.get("Navigation").unwrap().module_type {
        match target_name.clone() {
            Some(name) => {
                let target = masses_clone.get(&name);
                has_astroid_target = match target {
                    Some(target) => match target.mass_type {
                        MassType::Astroid{..} => true,
                        _ => false,
                    },
                    None => false,
                };
                is_within_range = match has_astroid_target {
                    true => match target {
                        Some(target) => mining_range > distance(ship.position, target.position),
                        _ => false,
                    }
                    _ => false,
                };
            }
            _ => (),
        }
    }

    MiningData {
        has_astroid_target  : has_astroid_target,
        is_within_range     : is_within_range,
        range               : mining_range,
        status              : mining_status,
    }
}
