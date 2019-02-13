extern crate serde_json;

use std::collections::HashMap;
use std::io::Write;

use crate::item::{Item, ItemType};
use crate::mass::{Mass, MassType};
use crate::modules::refinery::RefineryStatus;
use crate::server::connection::{receive, ServerConnection};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefineryData {
    pub has_crude_minerals: bool,
    pub status: RefineryStatus,
}

impl ServerConnection {
    pub fn server_refinery(&mut self, masses: &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();

        if let MassType::Ship {
            ref mut refinery,
            ref mut storage,
            ..
        } = ship.mass_type
        {
            let refinery = refinery.as_mut().unwrap();

            let refinery_data = RefineryData {
                has_crude_minerals: storage
                    .items
                    .iter()
                    .any(|item| item.itemtype == ItemType::CrudeMinerals),
                status: refinery.status.clone(),
            };

            let send = serde_json::to_string(&refinery_data).unwrap() + "\n";
            self.open = self.stream.write(send.as_bytes()).is_ok();

            match receive(&mut self.buff_r) {
                Some(recv) => {
                    if let "R" = recv.as_str() {
                        if refinery_data.has_crude_minerals {
                            refinery.toggle();
                        }
                    }
                }
                None => self.open = false,
            }

            if !refinery_data.has_crude_minerals {
                refinery.off();
            }

            if refinery.status == RefineryStatus::Refined {
                storage.take_item(ItemType::CrudeMinerals);
                storage.give_item(Item::new(ItemType::Iron));
                storage.give_item(Item::new(ItemType::Hydrogen));
                refinery.taken();
            }
        }

        masses.insert(self.name.clone(), ship);
    }
}
