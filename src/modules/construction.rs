use crate::modules::types::ModuleType;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ConstructionStatus {
    None,
    Constructing,
    Constructed,
}

impl Default for ConstructionStatus {
    fn default() -> Self {
        ConstructionStatus::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Construction {
    pub status: ConstructionStatus,
    construction: Option<ModuleType>,
    time: u64,
    start: Option<SystemTime>,
}

impl Construction {
    pub fn new() -> Construction {
        Construction {
            status: ConstructionStatus::None,
            construction: None,
            time: 5,
            start: None,
        }
    }

    pub fn process(&mut self) {
        if let Some(timer) = self.start {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.start = Some(SystemTime::now());
                self.status = ConstructionStatus::Constructed;
            }
        }
    }

    pub fn toggle(&mut self) {
        match self.status {
            ConstructionStatus::None => self.on(),
            _ => self.off(),
        };
    }

    pub fn on(&mut self) {
        self.start = Some(SystemTime::now());
        self.status = ConstructionStatus::Constructing;
    }

    pub fn off(&mut self) {
        self.start = None;
        self.status = ConstructionStatus::None;
    }

    pub fn take(&mut self) {
        self.off()
    }
}
