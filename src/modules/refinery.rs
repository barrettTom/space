use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RefineryStatus {
    None,
    Refining,
    Refined,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Refinery {
    time        : u64,
    start       : Option<SystemTime>,
    pub status  : RefineryStatus,
}

impl Refinery {
    pub fn new() -> Refinery {
        Refinery {
            time    : 5,
            start   : None,
            status  : RefineryStatus::None,
        }
    }

    pub fn process(&mut self) {
        match self.start.clone() {
            Some(timer) => {
                if timer.elapsed().unwrap().as_secs() > self.time {
                    self.status = RefineryStatus::Refined;
                    self.start = Some(SystemTime::now());
                }
            }
            _ => (),
        }
        if self.status == RefineryStatus::None {
            self.start = None;
        }
    }

    pub fn toggle(&mut self) {
        self.status = match self.status {
            RefineryStatus::None => {
                self.start = Some(SystemTime::now());
                RefineryStatus::Refining
            },
            _ => {
                self.start = None;
                RefineryStatus::None
            }
        };
    }

    pub fn off(&mut self) {
        self.status = RefineryStatus::None;
    }

    pub fn take(&mut self) {
        self.status = RefineryStatus::Refining;
    }
}
