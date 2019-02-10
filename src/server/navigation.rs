extern crate serde_json;

use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

use crate::mass::{Mass, MassType};
use crate::server::connection::ServerConnection;

impl ServerConnection {
    pub fn server_navigation(&mut self, masses: &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();

        if let MassType::Ship {
            ref mut navigation, ..
        } = ship.mass_type
        {
            let navigation = navigation.as_mut().unwrap();
            navigation.verify_target(ship_clone.position.clone(), &masses);
            let mut within_range: HashMap<&String, &Mass> = masses
                .iter()
                .filter(|&(_, mass)| {
                    ship_clone.position.distance_from(mass.position.clone()) < navigation.range
                })
                .collect();
            within_range.insert(&self.name, &ship_clone);

            if self.open {
                let send = serde_json::to_string(&within_range).unwrap() + "\n";

                if let Err(_err) = self.stream.write(send.as_bytes()) {
                    self.open = false;
                };

                let mut recv = String::new();
                if let Ok(result) = self.buff_r.read_line(&mut recv) {
                    if result == 0 {
                        self.open = false;
                    }
                    if !recv.is_empty() {
                        navigation.give_target(recv.replace("\n", ""));
                    }
                }
            }
        }

        masses.insert(self.name.clone(), ship);
    }
}
