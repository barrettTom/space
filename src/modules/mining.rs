use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MiningStatus {
    None,
    Mining,
    Mined,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mining {
    pub range       : f64,
    pub status      : MiningStatus,
    time            : u64,
    start           : Option<SystemTime>,
}

impl Mining {
    pub fn new() -> Mining {
        Mining {
            range   : 10.0,
            status  : MiningStatus::None,
            time    : 5,
            start   : None,
        }
    }

    pub fn process(&mut self) {
        if let Some(timer) = self.start.clone() {
            if timer.elapsed().unwrap().as_secs() > self.time {
                self.status = MiningStatus::Mined;
                self.start = Some(SystemTime::now());
            }
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

    pub fn take(&mut self) {
        self.status = MiningStatus::Mining;
    }
}
