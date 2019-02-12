use std::time::SystemTime;

use crate::constants;

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

    pub fn process(&mut self) {
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = RefineryStatus::Refined;
                self.start = Some(SystemTime::now());
            }
        }
    }

    pub fn toggle(&mut self) {
        match self.status {
            RefineryStatus::None => self.on(),
            _ => self.off(),
        };
    }

    pub fn on(&mut self) {
        self.start = Some(SystemTime::now());
        self.status = RefineryStatus::Refining;
    }

    pub fn off(&mut self) {
        self.start = None;
        self.status = RefineryStatus::None;
    }

    pub fn taken(&mut self) {
        self.status = RefineryStatus::Refining;
    }
}
