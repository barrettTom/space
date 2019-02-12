extern crate serde_json;

use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

use crate::item::{Item, ItemType};
use crate::mass::{Mass, MassType};
use crate::modules::refinery::RefineryStatus;
use crate::server::connection::ServerConnection;

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

            if self.open && self.txrx_refinery(&refinery_data) {
                refinery.toggle();
            }

            if !refinery_data.has_crude_minerals {
                refinery.off();
            }

            if refinery.status == RefineryStatus::Refined {
                storage.take_item(ItemType::CrudeMinerals);
                storage.give_item(Item::new(ItemType::Iron));
                refinery.taken();
            }
        }

        masses.insert(self.name.clone(), ship);
    }

    fn txrx_refinery(&mut self, refinery_data: &RefineryData) -> bool {
        let send = serde_json::to_string(refinery_data).unwrap() + "\n";
        if let Err(_err) = self.stream.write(send.as_bytes()) {
            self.open = false;
        }

        let mut recv = String::new();
        if let Ok(result) = self.buff_r.read_line(&mut recv) {
            match recv.as_bytes() {
                b"R\n" => {
                    if refinery_data.has_crude_minerals {
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
