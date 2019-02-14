extern crate serde_json;

use std::collections::HashMap;
use std::io::Write;

use crate::item::ItemType;
use crate::mass::{Mass, MassType};
use crate::math::Vector;
use crate::modules::mining::{Mining, MiningStatus};
use crate::server::connection::ServerConnection;

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
            if let Some(target_name) = navigation.target_name.clone() {
                let mut target = masses.remove(&target_name).unwrap();

                let mining_data = get_mining_data(ship.position.clone(), mining, target.clone());

                let send = serde_json::to_string(&mining_data).unwrap() + "\n";
                self.open = self.stream.write(send.as_bytes()).is_ok();

                let recv = self.receive();
                mining.give_recv(recv, mining_data);

                if mining.status == MiningStatus::Mined {
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
                    mining.mined();
                }
                masses.insert(target_name, target);
            }
        }

        masses.insert(self.name.clone(), ship);
    }
}

fn get_mining_data(position: Vector, mining: &Mining, target: Mass) -> MiningData {
    let mut astroid_has_minerals = false;
    let mut is_within_range = false;
    let has_astroid_target = match target.mass_type {
        MassType::Astroid { ref resources, .. } => {
            astroid_has_minerals = resources
                .items
                .iter()
                .any(|item| item.itemtype == ItemType::CrudeMinerals);
            is_within_range = mining.range > position.distance_from(target.position.clone());
            true
        }
        _ => false,
    };

    MiningData {
        has_astroid_target,
        astroid_has_minerals,
        is_within_range,
        range: mining.range,
        status: mining.status.clone(),
    }
}
