extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use math::distance;
use mass::{Mass, MassType};
use server::connection::ServerConnection;

impl ServerConnection {
    pub fn server_navigation(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let masses_clone = masses.clone();
        let ship = masses.get_mut(&self.name).unwrap();
        let ship_position = ship.position;

        if let MassType::Ship{ref mut navigation, ..} = ship.mass_type {
            let mut navigation = navigation.as_mut().unwrap();
            navigation.verify_target(ship_position, &masses_clone);
            let within_range : HashMap<&String, &Mass> = masses_clone.iter().filter(|&(_, mass)|
                                                                                    distance(ship_position, mass.position) < navigation.range)
                                                                                    .collect();
            let send = serde_json::to_string(&within_range).unwrap() + "\n";
            match self.stream.write(send.as_bytes()) {
                Ok(_result) => (),
                Err(_error) => return false,
            }

            let mut recv = String::new();
            match self.buff_r.read_line(&mut recv) {
                Ok(_result) => (),
                Err(_error) => (),
            }
            if !recv.is_empty() {
                navigation.give_target(recv.replace("\n", ""));
            }
        }
        true
    }
}
