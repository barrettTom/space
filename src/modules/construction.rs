use std::time::SystemTime;
use modules::types::ModuleType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ConstructionStatus {
    None,
    Constructing,
    Constructed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Construction {
    pub status      : ConstructionStatus,
    construction    : Option<ModuleType>,
    time            : u64,
    start           : Option<SystemTime>,
}

impl Construction {
    pub fn new() -> Construction {
        Construction {
            status  : ConstructionStatus::None,
            construction : None,
            time    : 5,
            start   : None,
        }
    }

    pub fn process(&mut self) {
        match self.start.clone() {
            Some(timer) => {
                if timer.elapsed().unwrap().as_secs() > self.time {
                    self.start = Some(SystemTime::now());
                    self.status = ConstructionStatus::Constructed;
                }
            }
            _ => (),
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
        self.status = ConstructionStatus::None;
    }
}
