extern crate serde_json;

use std::collections::HashMap;
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
            let refinery_data = RefineryData {
                has_crude_minerals: storage
                    .items
                    .iter()
                    .any(|item| item.itemtype == ItemType::CrudeMinerals),
                status: refinery.status.clone(),
            };

            let send = serde_json::to_string(&refinery_data).unwrap() + "\n";
            self.open = self.stream.write(send.as_bytes()).is_ok();

            let recv = self.receive();
            refinery.give_recv(recv, refinery_data);

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
