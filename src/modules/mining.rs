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
        match self.start.clone() {
            Some(timer) => {
                if timer.elapsed().unwrap().as_secs() > self.time {
                    self.status = MiningStatus::Mined;
                    self.start = Some(SystemTime::now());
                }
            }
            _ => (),
        }
        if self.status == MiningStatus::None {
            self.start = None;
        }
    }

    pub fn toggle(&mut self) {
        self.status = match self.status {
            MiningStatus::None => {
                self.start = Some(SystemTime::now());
                MiningStatus::Mining
            }
            _ => {
                self.start = None;
                MiningStatus::None
            }
        };
    }

    pub fn off(&mut self) {
        self.status = MiningStatus::None;
    }

    pub fn take(&mut self) {
        self.status = MiningStatus::Mining;
    }

}
