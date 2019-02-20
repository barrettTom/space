use std::time::SystemTime;

use crate::constants;
use crate::item::ItemType;
use crate::mass::{Mass, MassType};
use crate::math::Vector;
use crate::storage::Storage;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Mining {
    pub range: f64,
    pub status: Status,
    time: u64,
    start: Option<SystemTime>,
}

impl Mining {
    pub fn new() -> Mining {
        Mining {
            range: constants::SHIP_MINING_RANGE,
            status: Status::None,
            time: constants::SHIP_MINING_TIME,
            start: None,
        }
    }

    pub fn process(
        &mut self,
        ship_position: Vector,
        masses: &mut HashMap<String, Mass>,
        target: &mut Mass,
        storage: &mut Storage,
    ) {
        if self.range < ship_position.distance_from(target.position.clone()) {
            self.off();
        }
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = Status::Mined;
                self.start = None;
            }
        }
        if self.status == Status::Mined {
            if let MassType::Astroid {
                ref mut resources, ..
            } = target.mass_type
            {
                match resources.take_item(ItemType::CrudeMinerals) {
                    Some(item) => {
                        if !storage.give_item(item.clone()) {
                            let mass = Mass::new_item(
                                item.clone(),
                                target.position.clone(),
                                target.velocity.clone(),
                            );
                            masses.insert(item.name.clone(), mass);
                        }
                    }
                    None => self.off(),
                }
            }
            self.mined();
        }
    }

    pub fn give_received_data(&mut self, recv: String) {
        if let "F" = recv.as_str() {
            self.toggle()
        }
    }

    pub fn get_client_data(&self, ship_position: Vector, target: Option<&Mass>) -> String {
        let mut astroid_has_minerals = false;
        let mut is_within_range = false;
        let has_astroid_target = match target {
            Some(target) => match target.mass_type {
                MassType::Astroid { ref resources, .. } => {
                    astroid_has_minerals = resources
                        .items
                        .iter()
                        .any(|item| item.item_type == ItemType::CrudeMinerals);
                    is_within_range =
                        self.range > ship_position.distance_from(target.position.clone());
                    true
                }
                _ => false,
            },
            None => false,
        };

        let client_data = ClientData {
            has_astroid_target,
            astroid_has_minerals,
            is_within_range,
            range: self.range,
            status: self.status.clone(),
        };

        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    fn toggle(&mut self) {
        match self.status {
            Status::None => self.on(),
            _ => self.off(),
        };
    }

    fn on(&mut self) {
        self.start = Some(SystemTime::now());
        self.status = Status::Mining;
    }

    fn off(&mut self) {
        self.start = None;
        self.status = Status::None;
    }

    fn mined(&mut self) {
        self.status = Status::Mining;
        self.start = Some(SystemTime::now());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub has_astroid_target: bool,
    pub astroid_has_minerals: bool,
    pub is_within_range: bool,
    pub status: Status,
    pub range: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    None,
    Mining,
    Mined,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}
