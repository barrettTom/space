extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use math::distance;
use mass::{Mass, MassType};
use server::connection::ServerConnection;

impl ServerConnection {
    pub fn server_navigation(&mut self, masses : &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();

        if let MassType::Ship{ref mut navigation, ..} = ship.mass_type {
            let mut navigation = navigation.as_mut().unwrap();
            navigation.verify_target(ship_clone.position, &masses);
            let mut within_range : HashMap<&String, &Mass> = masses.iter().filter(|&(_, mass)|
                                                                                  distance(ship_clone.position, mass.position) < navigation.range)
                                                                                  .collect();
            within_range.insert(&self.name, &ship_clone);

            if self.open {
                let send = serde_json::to_string(&within_range).unwrap() + "\n";

                match self.stream.write(send.as_bytes()) {
                    Ok(_result) => (),
                    Err(_error) => self.open = false,
                };

                let mut recv = String::new();
                match self.buff_r.read_line(&mut recv) {
                    Ok(_result) => (),
                    Err(_error) => (),
                }
                if !recv.is_empty() {
                    navigation.give_target(recv.replace("\n", ""));
                }
            }
        }

        masses.insert(self.name.clone(), ship);
    }
}
