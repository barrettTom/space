extern crate serde_json;

use std::collections::HashMap;
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
            let construction_data = get_construction_data(storage, construction);

            let send = serde_json::to_string(&construction_data).unwrap() + "\n";
            self.open = self.stream.write(send.as_bytes()).is_ok();

            let recv = self.receive();
            construction.give_recv(recv, &construction_data);

            if construction_data.status == ConstructionStatus::Constructed {
                storage
                    .take_items(ItemType::Iron, constants::SHIP_CONSTRUCTION_IRON_COST)
                    .unwrap();
                masses.insert(
                    "Station".to_string(),
                    Mass::new_station(
                        ModuleType::Refinery,
                        ship.position.clone(),
                        ship.velocity.clone(),
                    ),
                );
                construction.constructed();
            }
        }

        masses.insert(self.name.clone(), ship);
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
