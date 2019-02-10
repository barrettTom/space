extern crate serde_json;

use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

use crate::mass::{Mass, MassType};
use crate::modules::navigation::NavigationStatus;
use crate::modules::tractorbeam::TractorbeamStatus;
use crate::server::connection::ServerConnection;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TractorbeamData {
    pub has_target: bool,
    pub status: TractorbeamStatus,
}

impl ServerConnection {
    pub fn server_tractorbeam(&mut self, masses: &mut HashMap<String, Mass>) {
        let mut ship = masses.remove(&self.name).unwrap();
        let ship_clone = ship.clone();

        if let MassType::Ship {
            ref mut tractorbeam,
            ref navigation,
            ..
        } = ship.mass_type
        {
            let tractorbeam = tractorbeam.as_mut().unwrap();
            let navigation = navigation.as_ref().unwrap();

            if self.open {
                let tractorbeam_data = TractorbeamData {
                    has_target: navigation.status == NavigationStatus::Targeted,
                    status: tractorbeam.status.clone(),
                };

                let send = serde_json::to_string(&tractorbeam_data).unwrap() + "\n";
                self.open = match self.stream.write(send.as_bytes()) {
                    Ok(_result) => true,
                    Err(_error) => false,
                };

                let mut recv = String::new();
                if let Ok(result) = self.buff_r.read_line(&mut recv) {
                    match recv.as_bytes() {
                        b"o\n" => tractorbeam.toggle_pull(),
                        b"p\n" => tractorbeam.toggle_push(),
                        b"t\n" => tractorbeam.toggle_bring(5.0),
                        _ => {
                            if result == 0 {
                                self.open = false;
                            }
                        }
                    }
                }
            }

            if let Some(name) = navigation.target_name.clone() {
                let target = masses.get_mut(&name).unwrap();
                let acceleration = tractorbeam.get_acceleration(ship_clone, target.clone());
                target.effects.give_acceleration(acceleration);
            } else {
                tractorbeam.off();
            }
        }

        masses.insert(self.name.clone(), ship);
    }
}
