use std::time::SystemTime;

use crate::constants;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Navigation {
    range: f64,
    status: Status,
    pub target: Option<Target>,
    time: u64,
    start: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Target {
    name: String,
    pub position: Vector,
    pub velocity: Vector,
}

impl Navigation {
    pub fn new() -> Navigation {
        Navigation {
            target: None,
            range: constants::SHIP_NAVIGATION_RANGE,
            status: Status::None,
            time: constants::SHIP_NAVIGATION_TIME,
            start: None,
        }
    }

    /*
    pub fn process(&mut self, ship_position: Vector, masses: &mut HashMap<String, Mass>) {
        self.verify_target(ship_position, masses);
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = Status::Targeted;
                self.start = None;
            }
        }
    }

    pub fn give_received_data(&mut self, recv: String) {
        if !recv.is_empty() {
            self.start = Some(SystemTime::now());
            self.status = Status::Targeting;
            self.target = Some(recv);
        }
    }

    pub fn get_client_data(&self, ship_position: Vector, masses: &HashMap<String, Mass>) -> String {
        let client_data = ClientData {
            ship_position: ship_position.clone(),
            status: self.status.clone(),
            target: self.target.clone(),
            available_targets: masses
                .iter()
                .filter(|(_, mass)| ship_position.distance_from(mass.position.clone()) < self.range)
                .map(|(name, mass)| (name.to_string(), mass.position.clone()))
                .collect(),
        };

        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    fn verify_target(&mut self, ship_position: Vector, masses: &HashMap<String, Mass>) {
        if let Some(name) = self.target.clone() {
            let good = match masses.get(&name) {
                Some(target) => {
                    target
                        .position
                        .distance_from(ship_position)
                        .partial_cmp(&self.range)
                        == Some(Ordering::Less)
                }
                None => false,
            };

            if !good {
                self.target = None;
                self.status = Status::None;
            }
        }
    }
    */
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub ship_position: Vector,
    pub available_targets: Vec<(String, Vector)>,
    pub status: Status,
    pub target: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    None,
    Targeting,
    Targeted,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}
