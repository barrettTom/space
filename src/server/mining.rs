extern crate serde_json;

use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

use math::distance;
use mass::{Mass, MassType};
use modules::mining::Mining;
use modules::navigation::Navigation;
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
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();
        let mut connection_good = true;

        if let MassType::Ship{ref mut mining, ref navigation, ..} = ship.mass_type {
            let mut mining = mining.as_mut().unwrap();
            let mut navigation = navigation.as_ref().unwrap();
            let mining_data = get_mining_data(ship_clone, mining, navigation, masses);

            let send = serde_json::to_string(&mining_data).unwrap() + "\n";
            match self.stream.write(send.as_bytes()) {
                Ok(_result) => (),
                Err(_error) => connection_good = false,
            }

            let mut recv = String::new();
            match self.buff_r.read_line(&mut recv) {
                Ok(result) => match recv.as_bytes() {
                    b"F\n" => {
                        if mining_data.is_within_range {
                            mining.toggle();
                        }
                    },
                    _ => {
                        if result == 0 {
                            connection_good = false;
                        }
                    },
                }
                Err(_error) => (),
            }
        }

        masses.insert(self.name.clone(), ship);
        connection_good
    }
}

fn get_mining_data(ship : Mass, mining : &Mining, navigation : &Navigation, masses : &mut HashMap<String, Mass>) -> MiningData {
    match navigation.target_name.clone() {
        Some(name) => {
            let target = masses.get(&name);
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

            MiningData {
                has_astroid_target  : has_astroid_target,
                is_within_range     : is_within_range,
                range               : mining.range,
                status              : mining.status,
            }
        }
        _ => {
            MiningData {
                has_astroid_target  : false,
                is_within_range     : false,
                range               : mining.range,
                status              : mining.status,
            }
        }
    }
}
