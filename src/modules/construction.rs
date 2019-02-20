use std::collections::HashMap;
use std::time::SystemTime;

use crate::constants;
use crate::item::ItemType;
use crate::mass::Mass;
use crate::math::Vector;
use crate::modules::types::ModuleType;
use crate::storage::Storage;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Construction {
    pub status: Status,
    construction: Option<ModuleType>,
    time: u64,
    start: Option<SystemTime>,
}

impl Construction {
    pub fn new() -> Construction {
        Construction {
            status: Status::None,
            construction: None,
            time: constants::SHIP_CONSTRUCTION_TIME,
            start: None,
        }
    }

    pub fn process(
        &mut self,
        ship_velocity: Vector,
        ship_position: Vector,
        masses: &mut HashMap<String, Mass>,
        storage: &mut Storage,
    ) {
        if !self.has_enough(storage) {
            self.off();
        }
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.start = Some(SystemTime::now());
                self.status = Status::Constructed;
            }
        }
        if self.status == Status::Constructed {
            storage
                .take_items(ItemType::Iron, constants::SHIP_CONSTRUCTION_IRON_COST)
                .unwrap();
            masses.insert(
                "Station".to_string(),
                Mass::new_station(
                    ModuleType::Refinery,
                    ship_position.clone(),
                    ship_velocity.clone(),
                ),
            );
            self.constructed();
        }
    }

    pub fn get_client_data(&self, storage: &Storage) -> String {
        let client_data = ClientData {
            has_enough: self.has_enough(storage),
            status: self.status.clone(),
        };
        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    pub fn give_received_data(&mut self, recv: String) {
        if let "c" = recv.as_str() {
            self.toggle()
        }
    }

    fn has_enough(&self, storage: &Storage) -> bool {
        storage
            .items
            .iter()
            .filter(|item| item.item_type == ItemType::Iron)
            .count()
            >= constants::SHIP_CONSTRUCTION_IRON_COST
    }

    fn toggle(&mut self) {
        match self.status {
            Status::None => self.on(),
            _ => self.off(),
        };
    }

    fn on(&mut self) {
        self.start = Some(SystemTime::now());
        self.status = Status::Constructing;
    }

    fn off(&mut self) {
        self.start = None;
        self.status = Status::None;
    }

    fn constructed(&mut self) {
        self.off()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub status: Status,
    pub has_enough: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    None,
    Constructing,
    Constructed,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}
