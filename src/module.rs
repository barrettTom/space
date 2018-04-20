use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NavigationStatus {
    None,
    Targeting,
    Targeted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Module {
    pub module_type : ModuleType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModuleType {
    Mining {
        range   : f64,
        status  : bool,
        time    : u64,
        start   : Option<SystemTime>,
    },
    Navigation {
        range       : f64,
        status      : NavigationStatus,
        time        : u64,
        start       : Option<SystemTime>,
        target_name : Option<String>,
    },
    Engines,
    Dashboard,
}

impl Module {
    pub fn new_mining() -> Module {
        let mining = ModuleType::Mining {
            range   : 10.0,
            status  : false,
            time    : 1,
            start   : None,
        };

        Module {
            module_type : mining,
        }
    }

    pub fn new_navigation() -> Module {
        let navigation = ModuleType::Navigation {
            target_name : None,
            range       : 100.0,
            status      : NavigationStatus::None,
            time        : 3,
            start       : None,
        };

        Module {
            module_type : navigation,
        }
    }

    pub fn new_dashboard() -> Module {
        Module {
            module_type : ModuleType::Dashboard,
        }
    }

    pub fn new_engines() -> Module {
        Module {
            module_type : ModuleType::Engines,
        }
    }

    pub fn process(&mut self) {
        match self.module_type {
            ModuleType::Navigation{ref mut status, ref mut start, ref time, ..} => {
                match start.clone() {
                    Some(timer) => {
                        if timer.elapsed().unwrap().as_secs() > *time {
                            *status = NavigationStatus::Targeted;
                            *start = None;
                        }
                    }
                    _ => (),
                }
            },
            ModuleType::Mining{..} => {
            },
            _ => (),
        }
    }
}
