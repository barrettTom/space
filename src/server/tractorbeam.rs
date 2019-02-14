extern crate serde_json;

use std::collections::HashMap;
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

        if let MassType::Ship {
            ref mut tractorbeam,
            ref navigation,
            ..
        } = ship.mass_type
        {
            if self.open {
                let tractorbeam_data = TractorbeamData {
                    has_target: navigation.status == NavigationStatus::Targeted,
                    status: tractorbeam.status.clone(),
                };

                let send = serde_json::to_string(&tractorbeam_data).unwrap() + "\n";
                self.open = self.stream.write(send.as_bytes()).is_ok();

                let recv = self.receive();
                tractorbeam.give_recv(recv);
            }

            if let Some(name) = navigation.target_name.clone() {
                let target = masses.get_mut(&name).unwrap();
                let acceleration =
                    tractorbeam.get_acceleration(ship.position.clone(), target.clone());
                target.effects.give_acceleration(acceleration);
            } else {
                tractorbeam.off();
            }
        }

        masses.insert(self.name.clone(), ship);
    }
}
