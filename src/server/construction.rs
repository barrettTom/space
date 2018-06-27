extern crate serde_json;

use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;

use mass::{Mass, MassType};
use modules::construction::Construction;
use server::connection::ServerConnection;
use modules::construction::ConstructionStatus;
use modules::types::ModuleType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstructionData {
    pub status      : ConstructionStatus,
    pub has_refined : bool,
}

impl ServerConnection {
    pub fn server_construction(&mut self, masses : &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();

        if let MassType::Ship{ref mut construction, ..} = ship.mass_type {
            let mut construction = construction.as_mut().unwrap();
            let construction_data = get_construction_data(ship_clone.clone(), construction);

            if self.open {
                if self.txrx_construction(&construction_data) {
                    construction.toggle();
                }
            }

            if construction_data.status == ConstructionStatus::Constructed {
                construction.take();
                masses.insert("Station".to_string(), Mass::new_station(ModuleType::Refinery, ship_clone.position, ship_clone.velocity));
            }
        }

        masses.insert(self.name.clone(), ship);
    }

    fn txrx_construction(&mut self, construction_data : &ConstructionData) -> bool {
        let send = serde_json::to_string(construction_data).unwrap() + "\n";
        match self.stream.write(send.as_bytes()) {
            Err(_error) => self.open = false,
            _ => (),
        }

        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(result) => match recv.as_bytes() {
                b"c\n" => {
                    if construction_data.has_refined {
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

fn get_construction_data(ship : Mass, construction : &Construction) -> ConstructionData {
    let mut has_refined = false;
    if ship.refined_count() >= 5 {
        has_refined = true;
    }

    ConstructionData {
        status      : construction.status.clone(),
        has_refined : has_refined,
    }
}
