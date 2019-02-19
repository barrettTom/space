use std::time::SystemTime;

use crate::constants;
use crate::item::{Item, ItemType};
use crate::storage::Storage;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Refinery {
    time: u64,
    start: Option<SystemTime>,
    pub status: RefineryStatus,
}

impl Refinery {
    pub fn new() -> Refinery {
        Refinery {
            time: constants::SHIP_REFINERY_TIME,
            start: None,
            status: RefineryStatus::None,
        }
    }

    pub fn process(&mut self, storage: &mut Storage) {
        if !self.has_crude_minerals(storage) {
            self.off();
        }
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = RefineryStatus::Refined;
                self.start = None
            }
        }
        if self.status == RefineryStatus::Refined {
            storage.take_item(ItemType::CrudeMinerals);
            storage.give_item(Item::new(ItemType::Iron));
            storage.give_item(Item::new(ItemType::Hydrogen));
            self.taken();
        }
    }

    pub fn get_client_data(&self, storage: &Storage) -> String {
        let client_data = RefineryClientData {
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
            RefineryStatus::None => self.on(),
            _ => self.off(),
        };
    }

    fn on(&mut self) {
        self.start = Some(SystemTime::now());
        self.status = RefineryStatus::Refining;
    }

    fn off(&mut self) {
        self.start = None;
        self.status = RefineryStatus::None;
    }

    fn taken(&mut self) {
        self.status = RefineryStatus::Refining;
        self.start = Some(SystemTime::now());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefineryClientData {
    pub has_crude_minerals: bool,
    pub status: RefineryStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RefineryStatus {
    None,
    Refining,
    Refined,
}

impl Default for RefineryStatus {
    fn default() -> Self {
        RefineryStatus::None
    }
}
