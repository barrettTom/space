extern crate serde_json;

use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

use math::distance;
use mass::{Mass, MassType};
use modules::navigation::Navigation;
use server::connection::ServerConnection;
use modules::mining::{Mining, MiningStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiningData {
    pub has_astroid_target  : bool,
    pub astroid_has_minerals: bool,
    pub is_within_range     : bool,
    pub status              : MiningStatus,
    pub range               : f64,
}

impl ServerConnection {
    pub fn server_mining(&mut self, masses : &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();
        let mut item = None;

        if let MassType::Ship{ref mut mining, ref navigation, ..} = ship.mass_type {
            let mut mining = mining.as_mut().unwrap();
            let mut navigation = navigation.as_ref().unwrap();
            let mining_data = get_mining_data(ship_clone, mining, navigation, masses);

            if self.open {
                if self.txrx_mining(&mining_data) {
                    mining.toggle();
                }
            }

            if !mining_data.is_within_range {
                mining.off();
            }
            else {
                if mining.status == MiningStatus::Mined {
                    mining.take();
                    if let Some(name) = navigation.target_name.clone() {
                        let target = masses.get_mut(&name).unwrap();
                        item = target.take("Mineral");
                    }
                }
            }
        }

        if let Some(item) = item {
            if !ship.give(item.clone()) {
                let mass = Mass::new_item(item.clone(), ship.position, ship.velocity);
                masses.insert(item.name.clone(), mass);
            }
        }

        masses.insert(self.name.clone(), ship);
    }

    fn txrx_mining(&mut self, mining_data : &MiningData) -> bool {
        let send = serde_json::to_string(mining_data).unwrap() + "\n";
        if let Err(_err) = self.stream.write(send.as_bytes()) {
            self.open = false;
        }

        let mut recv = String::new();
        if let Ok(result) = self.buff_r.read_line(&mut recv) {
            match recv.as_bytes() {
                b"F\n" => {
                    if mining_data.is_within_range {
                        return true;
                    }
                },
                _ => {
                    if result == 0 {
                        self.open = false;
                    }
                },
            }
        }
        false
    }
}

fn get_mining_data(ship : Mass, mining : &Mining, navigation : &Navigation, masses : &mut HashMap<String, Mass>) -> MiningData {
    match navigation.target_name.clone() {
        Some(name) => {
            let target = masses.get(&name);

            let mut astroid_has_minerals = false;
            let has_astroid_target = match target {
                Some(target) => {
                    astroid_has_minerals = target.has_minerals();
                    match target.mass_type {
                        MassType::Astroid{..} => true,
                        _ => false,
                    }
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
                astroid_has_minerals: astroid_has_minerals,
                is_within_range     : is_within_range,
                range               : mining.range,
                status              : mining.status.clone(),
            }
        }
        _ => {
            MiningData {
                has_astroid_target  : false,
                astroid_has_minerals: false,
                is_within_range     : false,
                range               : mining.range,
                status              : mining.status.clone(),
            }
        }
    }
}
