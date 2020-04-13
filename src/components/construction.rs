use std::collections::HashMap;
use std::time::SystemTime;

use crate::components::item::ItemType;
use crate::components::storage::Storage;
use crate::components::types::ModuleType;
use crate::constants;
use crate::mass::Mass;
use crate::math::Vector;

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
        if storage.item_count(ItemType::Iron) < constants::SHIP_CONSTRUCTION_IRON_COST {
            self.off();
        }
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.start = Some(SystemTime::now());
                self.status = Status::Constructed;
            }
        }
        if self.status == Status::Constructed {
            for _ in 0..constants::SHIP_CONSTRUCTION_IRON_COST {
                storage.take_item(ItemType::Iron).unwrap();
            }

            masses.insert(
                "Station".to_string(),
                Mass::new_station(ModuleType::Refinery, ship_position, ship_velocity),
            );
            self.constructed();
        }
    }

    pub fn get_client_data(&self, storage: &Storage) -> String {
        let client_data = ClientData {
            has_enough: storage.item_count(ItemType::Iron)
                >= constants::SHIP_CONSTRUCTION_IRON_COST,
            status: self.status.clone(),
        };
        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    pub fn give_received_data(&mut self, recv: String) {
        if let "c" = recv.as_str() {
            self.toggle()
        }
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
