extern crate serde_json;

use std::collections::HashMap;
use std::io::Write;

use crate::mass::{Mass, MassType};
use crate::modules::navigation::NavigationStatus;
use crate::server::connection::{receive, ServerConnection};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnginesData {
    pub has_target: bool,
    pub fuel: f64,
}

impl ServerConnection {
    pub fn server_engines(&mut self, masses: &mut HashMap<String, Mass>) {
        if self.open {
            let mut ship = masses.remove(&self.name).unwrap();

            if let MassType::Ship {
                ref mut engines,
                ref navigation,
                ..
            } = ship.mass_type
            {
                let navigation = navigation.clone().unwrap();
                let engines = engines.as_mut().unwrap();

                let engines_data = EnginesData {
                    has_target: navigation.status == NavigationStatus::Targeted,
                    fuel: engines.fuel,
                };
                let send = serde_json::to_string(&engines_data).unwrap() + "\n";
                self.open = self.stream.write(send.as_bytes()).is_ok();

                let target = match navigation.target_name {
                    Some(name) => masses.get(&name),
                    None => None,
                };

                match receive(&mut self.buff_r) {
                    Some(recv) => engines.give_client_data(
                        ship.position.clone(),
                        ship.velocity.clone(),
                        target,
                        recv,
                    ),
                    None => self.open = false,
                }
            }

            masses.insert(self.name.clone(), ship);
        }
    }
}
