use std::time::SystemTime;
use std::collections::HashMap;

use mass::Mass;
use math::distance;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModuleType {
    Navigation,
    Mining,
    Engines,
    Dashboard,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NavigationStatus {
    None,
    Targeting,
    Targeted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Navigation {
    pub range       : f64,
    pub status      : NavigationStatus,
    pub target_name : Option<String>,
    time            : u64,
    start           : Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mining {
    pub range       : f64,
    pub status      : bool,
    time            : u64,
    start           : Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Engines {
    acceleration : (f64, f64, f64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dashboard {}

impl Mining {
    pub fn new() -> Mining {
        Mining {
            range   : 10.0,
            status  : false,
            time    : 1,
            start   : None,
        }
    }

    pub fn toggle(&self) {
    }
}

impl Navigation {
    pub fn new() -> Navigation {
        Navigation {
            target_name : None,
            range       : 100.0,
            status      : NavigationStatus::None,
            time        : 3,
            start       : None,
        }
    }

    pub fn process(&mut self) {
        match self.start.clone() {
            Some(timer) => {
                if timer.elapsed().unwrap().as_secs() > self.time {
                    self.status = NavigationStatus::Targeted;
                    self.start = None;
                }
            }
            _ => (),
        }
    }

    pub fn give_target(&mut self, target_name : String) {
        self.start = Some(SystemTime::now());
        self.status = NavigationStatus::Targeting;
        self.target_name = Some(target_name);
    }

    pub fn verify_target(&mut self, ship_position : (f64, f64, f64), masses : &HashMap<String, Mass>) {
        match self.target_name.clone() {
            Some(name) => {
                let target = masses.get(&name).unwrap();
                if distance(target.position, ship_position) > self.range {
                    self.target_name = None;
                    self.status = NavigationStatus::None;
                }
            }
            _ => (),
        }
    }
}

impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard {}
    }
}

impl Engines {
    pub fn new() -> Engines {
        Engines {
            acceleration : (0.0, 0.0, 0.0)
        }
    }

    pub fn recv_acceleration(&mut self) -> (f64, f64, f64) {
        let acceleration = self.acceleration;
        self.acceleration = (0.0, 0.0, 0.0);
        acceleration
    }

    pub fn give_client_data(&mut self, ship : &Mass, target : Option<&Mass>, data : String) {
        let mut acceleration = (0.0, 0.0, 0.0);
        match data.as_bytes() {
            b"5\n" => acceleration.0 += 0.1,
            b"0\n" => acceleration.0 -= 0.1,
            b"8\n" => acceleration.1 += 0.1,
            b"2\n" => acceleration.1 -= 0.1,
            b"4\n" => acceleration.2 += 0.1,
            b"6\n" => acceleration.2 -= 0.1,
            b"+\n" => {
                let m_v = ship.velocity;
                acceleration = (m_v.0 * 0.05,
                                m_v.1 * 0.05,
                                m_v.2 * 0.05);
            },
            b"-\n" => {
                let m_v = ship.velocity;
                acceleration = (-1.0 * m_v.0 * 0.05,
                                -1.0 * m_v.1 * 0.05,
                                -1.0 * m_v.2 * 0.05);
            },
            b"c\n" => {
                match target {
                    Some(target) => {
                        let d_v = target.velocity;
                        let m_v = ship.velocity;
                        acceleration = (d_v.0 - m_v.0,
                                        d_v.1 - m_v.1,
                                        d_v.2 - m_v.2);
                    },
                    None => (),
                }
            },
            b"t\n" => {
                match target {
                    Some(target) => {
                        let d_p = target.position;
                        let m_p = ship.position;
                        acceleration = ((d_p.0 - m_p.0) * 0.01,
                                        (d_p.1 - m_p.1) * 0.01,
                                        (d_p.2 - m_p.2) * 0.01);
                    },
                    None => (),
                }
            },
            _ => (),
        }
        self.acceleration = acceleration;
    }
}
