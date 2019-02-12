extern crate serde_json;

use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

use crate::constants;
use crate::item::ItemType;
use crate::mass::{Mass, MassType};
use crate::modules::construction::Construction;
use crate::modules::construction::ConstructionStatus;
use crate::modules::types::ModuleType;
use crate::server::connection::ServerConnection;
use crate::storage::Storage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstructionData {
    pub status: ConstructionStatus,
    pub has_enough: bool,
}

impl ServerConnection {
    pub fn server_construction(&mut self, masses: &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();

        if let MassType::Ship {
            ref mut construction,
            ref mut storage,
            ..
        } = ship.mass_type
        {
            let construction = construction.as_mut().unwrap();
            let construction_data = get_construction_data(storage, construction);

            if self.open && self.txrx_construction(&construction_data) {
                construction.toggle();
            }

            if construction_data.status == ConstructionStatus::Constructed {
                storage.take_items(ItemType::Iron, constants::SHIP_CONSTRUCTION_IRON_COST);
                masses.insert(
                    "Station".to_string(),
                    Mass::new_station(
                        ModuleType::Refinery,
                        ship.position.clone(),
                        ship.velocity.clone(),
                    ),
                );
                construction.taken();
            }
        }

        masses.insert(self.name.clone(), ship);
    }

    fn txrx_construction(&mut self, construction_data: &ConstructionData) -> bool {
        let send = serde_json::to_string(construction_data).unwrap() + "\n";
        if let Err(_err) = self.stream.write(send.as_bytes()) {
            self.open = false;
        }

        let mut recv = String::new();
        if let Ok(result) = self.buff_r.read_line(&mut recv) {
            match recv.as_bytes() {
                b"c\n" => {
                    if construction_data.has_enough {
                        return true;
                    }
                }
                _ => {
                    if result == 0 {
                        self.open = false;
                    }
                }
            }
        }

        false
    }
}

fn get_construction_data(storage: &Storage, construction: &Construction) -> ConstructionData {
    ConstructionData {
        status: construction.status.clone(),
        has_enough: storage
            .items
            .iter()
            .filter(|item| item.itemtype == ItemType::Iron)
            .count()
            >= constants::SHIP_CONSTRUCTION_IRON_COST,
    }
}
