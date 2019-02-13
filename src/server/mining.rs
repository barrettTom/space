extern crate serde_json;

use std::collections::HashMap;
use std::io::Write;

use crate::item::ItemType;
use crate::mass::{Mass, MassType};
use crate::math::Vector;
use crate::modules::mining::{Mining, MiningStatus};
use crate::modules::navigation::Navigation;
use crate::server::connection::{receive, ServerConnection};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiningData {
    pub has_astroid_target: bool,
    pub astroid_has_minerals: bool,
    pub is_within_range: bool,
    pub status: MiningStatus,
    pub range: f64,
}

impl ServerConnection {
    pub fn server_mining(&mut self, masses: &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();

        if let MassType::Ship {
            ref mut mining,
            ref mut storage,
            ref navigation,
            ..
        } = ship.mass_type
        {
            let mining = mining.as_mut().unwrap();
            let navigation = navigation.as_ref().unwrap();
            let mining_data = get_mining_data(ship.position.clone(), mining, navigation, masses);

            let send = serde_json::to_string(&mining_data).unwrap() + "\n";
            self.open = self.stream.write(send.as_bytes()).is_ok();

            match receive(&mut self.buff_r) {
                Some(recv) => {
                    if let "F" = recv.as_str() {
                        if mining_data.is_within_range {
                            mining.toggle();
                        }
                    }
                }
                None => self.open = false,
            }

            if !mining_data.is_within_range {
                mining.off();
            } else if mining.status == MiningStatus::Mined {
                if let Some(name) = navigation.target_name.clone() {
                    let target = masses.get_mut(&name).unwrap();
                    if let MassType::Astroid {
                        ref mut resources, ..
                    } = target.mass_type
                    {
                        match resources.take_item(ItemType::CrudeMinerals) {
                            Some(item) => {
                                if !storage.give_item(item.clone()) {
                                    let mass = Mass::new_item(
                                        item.clone(),
                                        ship.position.clone(),
                                        ship.velocity.clone(),
                                    );
                                    masses.insert(item.name.clone(), mass);
                                }
                            }
                            None => mining.off(),
                        }
                    }
                }
                mining.taken();
            }
        }

        masses.insert(self.name.clone(), ship);
    }
}

fn get_mining_data(
    position: Vector,
    mining: &Mining,
    navigation: &Navigation,
    masses: &mut HashMap<String, Mass>,
) -> MiningData {
    match navigation.target_name.clone() {
        Some(name) => {
            let target = masses.get(&name);

            let mut astroid_has_minerals = false;
            let has_astroid_target = match target {
                Some(target) => match target.mass_type {
                    MassType::Astroid { ref resources, .. } => {
                        astroid_has_minerals = resources
                            .items
                            .iter()
                            .any(|item| item.itemtype == ItemType::CrudeMinerals);
                        true
                    }
                    _ => false,
                },
                None => false,
            };

            let is_within_range = if has_astroid_target {
                match target {
                    Some(target) => mining.range > position.distance_from(target.position.clone()),
                    _ => false,
                }
            } else {
                false
            };

            MiningData {
                has_astroid_target,
                astroid_has_minerals,
                is_within_range,
                range: mining.range,
                status: mining.status.clone(),
            }
        }
        _ => MiningData {
            has_astroid_target: false,
            astroid_has_minerals: false,
            is_within_range: false,
            range: mining.range,
            status: mining.status.clone(),
        },
    }
}
