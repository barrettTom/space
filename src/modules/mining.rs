use std::time::SystemTime;

use crate::constants;
use crate::server::mining::MiningData;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MiningStatus {
    None,
    Mining,
    Mined,
}

impl Default for MiningStatus {
    fn default() -> Self {
        MiningStatus::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Mining {
    pub range: f64,
    pub status: MiningStatus,
    time: u64,
    start: Option<SystemTime>,
}

impl Mining {
    pub fn new() -> Mining {
        Mining {
            range: constants::SHIP_MINING_RANGE,
            status: MiningStatus::None,
            time: constants::SHIP_MINING_TIME,
            start: None,
        }
    }

    pub fn process(&mut self) {
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = MiningStatus::Mined;
                self.start = Some(SystemTime::now());
            }
        }
    }

    pub fn give_recv(&mut self, recv: String, mining_data: MiningData) {
        if !mining_data.is_within_range {
            self.off();
        } else if let "F" = recv.as_str() {
            self.toggle()
        }
    }

    pub fn toggle(&mut self) {
        match self.status {
            MiningStatus::None => self.on(),
            _ => self.off(),
        };
    }

    pub fn on(&mut self) {
        self.start = Some(SystemTime::now());
        self.status = MiningStatus::Mining;
    }

    pub fn off(&mut self) {
        self.start = None;
        self.status = MiningStatus::None;
    }

    pub fn mined(&mut self) {
        self.status = MiningStatus::Mining;
    }
}
