extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use item::Item;
use mass::{Mass, MassType};
use server::connection::ServerConnection;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefineryData {
    pub has_minerals    : bool,
    pub status          : bool,
}

impl ServerConnection {
    pub fn server_refinery(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();
        let mut refine = false;
        let mut connection_good = true;

        if let MassType::Ship{ref mut refinery, ..} = ship.mass_type {
            let mut refinery = refinery.as_mut().unwrap();

            let refinery_data = RefineryData {
                has_minerals    : ship_clone.has_minerals(),
                status          : refinery.status,
            };

            let send = serde_json::to_string(&refinery_data).unwrap() + "\n";
            match self.stream.write(send.as_bytes()) {
                Ok(_result) => (),
                Err(_error) => connection_good = false,
            }

            let mut recv = String::new();
            match self.buff_r.read_line(&mut recv) {
                Ok(result) => match recv.as_bytes() {
                    b"R\n" => {
                        if refinery_data.has_minerals {
                            refinery.toggle();
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

            if !refinery_data.has_minerals {
                refinery.off();
            }

            if refinery.status && refinery.ready {
                refinery.take();
                refine = true;
            }
        }
    
        if refine {
            ship.take("Iron");
            ship.give(Item::new("Refined Iron", 1));
        }

        masses.insert(self.name.clone(), ship);
        connection_good
    }
}
