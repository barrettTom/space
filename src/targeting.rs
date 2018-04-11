use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TargetingStatus {
    None,
    Targeting,
    Targeted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Targeting {
    target  : Option<String>,
    status  : TargetingStatus,
    time    : u64,
    start   : Option<SystemTime>,
}

impl Targeting {
    pub fn new() -> Targeting {
        Targeting {
            target : None,
            status : TargetingStatus::None,
            time   : 3,
            start  : None,
        }
    }

    pub fn process(&mut self) {
        match self.start {
            Some(time) => {
                if time.elapsed().unwrap().as_secs() > self.time {
                    self.status = TargetingStatus::Targeted;
                    self.start = None;
                }
            }
            None => (),
        }
    }
    pub fn give_target(&mut self, target : Option<String>) {
        self.target = target;
        match self.target {
            Some(_) => {
                self.status = TargetingStatus::Targeting;
                self.start = Some(SystemTime::now());
            },
            None => {
                self.status = TargetingStatus::None;
                self.start = None;
            }
        }
    }

    pub fn recv_target(&self) -> Option<String> {
        self.target.clone()
    }

    pub fn recv_status(&self) -> TargetingStatus {
        self.status.clone()
    }
}
