use std::time::SystemTime;

use crate::components::item::{Item, ItemType};
use crate::components::storage::Storage;
use crate::constants;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Refinery {
    time: u64,
    start: Option<SystemTime>,
    pub status: Status,
}

impl Refinery {
    pub fn new() -> Refinery {
        Refinery {
            time: constants::SHIP_REFINERY_TIME,
            start: None,
            status: Status::None,
        }
    }

    pub fn process(&mut self, storage: &mut Storage) {
        if !self.has_crude_minerals(storage) {
            self.off();
        }
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = Status::Refined;
                self.start = None
            }
        }
        if self.status == Status::Refined {
            storage.take_item(ItemType::CrudeMinerals);
            storage.give_item(Item::new(ItemType::Iron));
            storage.give_item(Item::new(ItemType::Hydrogen));
            self.taken();
        }
    }

    pub fn get_client_data(&self, storage: &Storage) -> String {
        let client_data = ClientData {
            has_crude_minerals: self.has_crude_minerals(storage),
            status: self.status.clone(),
        };

        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    pub fn give_received_data(&mut self, recv: String) {
        if let "R" = recv.as_str() {
            self.toggle();
        }
    }

    fn has_crude_minerals(&self, storage: &Storage) -> bool {
        storage
            .items
            .iter()
            .any(|item| item.item_type == ItemType::CrudeMinerals)
    }

    fn toggle(&mut self) {
        match self.status {
            Status::None => self.on(),
            _ => self.off(),
        };
    }

    fn on(&mut self) {
        self.start = Some(SystemTime::now());
        self.status = Status::Refining;
    }

    fn off(&mut self) {
        self.start = None;
        self.status = Status::None;
    }

    fn taken(&mut self) {
        self.status = Status::Refining;
        self.start = Some(SystemTime::now());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub has_crude_minerals: bool,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    None,
    Refining,
    Refined,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}
