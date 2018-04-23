use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mining {
    pub range       : f64,
    pub status      : bool,
    time            : u64,
    start           : Option<SystemTime>,
}


impl Mining {
    pub fn new() -> Mining {
        Mining {
            range   : 10.0,
            status  : false,
            time    : 1,
            start   : None,
        }
    }

    pub fn toggle(&self) {}
}
