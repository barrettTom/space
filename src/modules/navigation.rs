use std::collections::HashMap;
use std::time::SystemTime;

use crate::mass::Mass;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NavigationStatus {
    None,
    Targeting,
    Targeted,
}

impl Default for NavigationStatus {
    fn default() -> Self {
        NavigationStatus::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Navigation {
    pub range: f64,
    pub status: NavigationStatus,
    pub target_name: Option<String>,
    time: u64,
    start: Option<SystemTime>,
}

impl Navigation {
    pub fn new() -> Navigation {
        Navigation {
            target_name: None,
            range: 100.0,
            status: NavigationStatus::None,
            time: 3,
            start: None,
        }
    }

    pub fn process(&mut self) {
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = NavigationStatus::Targeted;
                self.start = None;
            }
        }
    }

    pub fn give_target(&mut self, target_name: String) {
        self.start = Some(SystemTime::now());
        self.status = NavigationStatus::Targeting;
        self.target_name = Some(target_name);
    }

    pub fn verify_target(&mut self, ship_position: Vector, masses: &HashMap<String, Mass>) {
        if let Some(name) = self.target_name.clone() {
            let target = masses.get(&name).unwrap();
            if target.position.distance_from(ship_position) > self.range {
                self.target_name = None;
                self.status = NavigationStatus::None;
            }
        }
    }
}
