extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use item::Item;
use mass::{Mass, MassType};
use server::connection::ServerConnection;
use modules::refinery::RefineryStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefineryData {
    pub has_minerals    : bool,
    pub status          : RefineryStatus,
}

impl ServerConnection {
    pub fn server_refinery(&mut self, masses : &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();
        let mut refine = false;

        if let MassType::Ship{ref mut refinery, ..} = ship.mass_type {
            let mut refinery = refinery.as_mut().unwrap();

            let refinery_data = RefineryData {
                has_minerals    : ship_clone.has_minerals(),
                status          : refinery.status.clone(),
            };

            if self.open {
                if self.txrx_refinery(&refinery_data) {
                    refinery.toggle();
                }
            }

            if !refinery_data.has_minerals {
                refinery.off();
            }

            if refinery.status == RefineryStatus::Refined {
                refinery.take();
                refine = true;
            }
        }
    
        if refine {
            ship.take("Mineral");
            ship.give(Item::new("Refined Mineral", 1));
        }

        masses.insert(self.name.clone(), ship);
    }

    fn txrx_refinery(&mut self, refinery_data : &RefineryData) -> bool {
        let send = serde_json::to_string(refinery_data).unwrap() + "\n";
        match self.stream.write(send.as_bytes()) {
            Err(_error) => self.open = false,
            _ => (),
        }

        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(result) => match recv.as_bytes() {
                b"R\n" => {
                    if refinery_data.has_minerals {
                        return true
                    }
                },
                _ => {
                    if result == 0 {
                        self.open = false;
                    }
                },
            }
            _ => (),
        }

        false
    }
}

