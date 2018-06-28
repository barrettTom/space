extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use mass::{Mass, MassType};
use modules::navigation::NavigationStatus;
use server::connection::ServerConnection;

impl ServerConnection {
    pub fn server_engines(&mut self, masses : &mut HashMap<String, Mass>) {
        if self.open {
            let mut ship = masses.remove(&self.name).unwrap();
            let ship_clone = ship.clone();

            if let MassType::Ship{ref mut engines, ref navigation, ..} = ship.mass_type {
                let navigation = navigation.clone().unwrap();
                let engines = engines.as_mut().unwrap();
                let targeted = navigation.status == NavigationStatus::Targeted;

                let send = serde_json::to_string(&targeted).unwrap() + "\n";
                if let Err(_err) = self.stream.write(send.as_bytes()) {
                    self.open = false;
                }

                let target = match navigation.target_name {
                    Some(name) => masses.get(&name),
                    None => None,
                };

                let mut recv = String::new();
                if let Ok(result) = self.buff_r.read_line(&mut recv) {
                    engines.give_client_data(&ship_clone, target, recv);
                    if result == 0 {
                        self.open = false;
                    }
                }
            }

            masses.insert(self.name.clone(), ship);
        }
    }
}
