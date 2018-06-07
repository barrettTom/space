use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Refinery {
    time        : u64,
    start       : Option<SystemTime>,
    pub status  : bool,
    pub ready   : bool,
}

impl Refinery {
    pub fn new() -> Refinery {
        Refinery {
            time    : 5,
            start   : None,
            status  : false,
            ready   : false,
        }
    }

    pub fn process(&mut self) {
        match self.start.clone() {
            Some(timer) => {
                if timer.elapsed().unwrap().as_secs() > self.time {
                    self.start = Some(SystemTime::now());
                    self.ready = true;
                }
            }
            _ => (),
        }
        if !self.status {
            self.start = None;
            self.ready = false;
        }
    }

    pub fn toggle(&mut self) {
        self.status = !self.status;
        self.start = match self.status {
            true => Some(SystemTime::now()),
            false => None,
        };
    }

    pub fn off(&mut self) {
        self.status = false;
    }

    pub fn take(&mut self) {
        self.ready = false;
    }
}
