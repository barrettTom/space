extern crate serde_json;

use std::time::SystemTime;
//use std::collections::HashMap;

use storage::Storage;
use mass::{Mass, MassType};
//use module::{Module, ModuleType};
use module::ModuleType;
use targeting::{Targeting, TargetingStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    mass_type   : MassType,
    position    : (f64, f64, f64),
    velocity    : (f64, f64, f64),
    range       : f64,
    modules     : Vec<ModuleType>,
    mining      : Mining,
    targeting   : Targeting,
//    modules     : HashMap<ModuleType, Box<Module>>,
    storage     : Storage,
}

impl Ship {
    pub fn new(position : (f64, f64, f64)) -> Ship {
        let mut modules = Vec::new();

        modules.push(ModuleType::Navigation);
        modules.push(ModuleType::Engines);
        modules.push(ModuleType::Dashboard);
        modules.push(ModuleType::Mining);

//        let mut modules : HashMap<ModuleType, Box<Module>> = HashMap::new();
//        modules.insert(ModuleType::Navigation, Targeting::new());
//        modules.insert(ModuleType::Mining, Mining::new());

        Ship {
            mass_type   : MassType::Ship,
            position    : position,
            velocity    : (0.0, 0.0, 0.0),
            range       : 100.0,
            modules     : modules,
            mining      : Mining::new(),
            targeting   : Targeting::new(),
            storage     : Storage::new(Vec::new()),
        }
    }

    pub fn slow(&mut self) {
        if self.velocity.0 > 0.01 {
            self.velocity.0 += -1.0 * self.velocity.0 * 0.1;
        }
        else {
            self.velocity.0 = 0.0;
        }

        if self.velocity.1 > 0.01 {
            self.velocity.1 += -1.0 * self.velocity.1 * 0.1;
        }
        else {
            self.velocity.1 = 0.0;
        }

        if self.velocity.2 > 0.01 {
            self.velocity.2 += -1.0 * self.velocity.2 * 0.1;
        }
        else {
            self.velocity.2 = 0.0;
        }
    }

    pub fn speedup(&mut self) {
        self.velocity.0 *= 1.05;
        self.velocity.1 *= 1.05;
        self.velocity.2 *= 1.05;
    }
    pub fn start_mining(&mut self) {
        self.mining.start()
    }

    pub fn stop_mining(&mut self) {
        self.mining.stop()
    }

    pub fn recv_range(&self) -> f64 {
        self.range
    }

    pub fn recv_mining_range(&self) -> f64 {
        self.mining.recv_range()
    }

    pub fn recv_mining_status(&self) -> bool {
        self.mining.recv_status()
    }

    pub fn give_target(&mut self, target : Option<String>) {
        self.targeting.give_target(target);
    }

    pub fn recv_target(&self) -> Option<String> {
        self.targeting.recv_target()
    }

    pub fn recv_targeting_status(&self) -> TargetingStatus {
        self.targeting.recv_status()
    }

    pub fn recv_modules(&self) -> String {
        serde_json::to_string(&self.modules).unwrap() + "\n"
    }
}

impl Mass for Ship {
    fn recv_mass_type(&self) -> MassType {
        self.mass_type.clone()
    }

    fn process(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
        self.targeting.process()
    }

    fn position(&self) -> (f64, f64, f64) {
        self.position
    }

    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn box_clone(&self) -> Box<Mass> {
        Box::new((*self).clone())
    }

    fn recv_velocity(&self) -> (f64, f64, f64) {
        self.velocity
    }

    fn give_acceleration(&mut self, acceleration : (f64, f64, f64)) {
        self.velocity.0 += acceleration.0;
        self.velocity.1 += acceleration.1;
        self.velocity.2 += acceleration.2;
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Mining {
    range   : f64,
    status  : bool,
    time    : u64,
    start   : Option<SystemTime>,
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

    pub fn start(&mut self) {
        self.status = true;
    }

    pub fn stop(&mut self) {
        self.status = false;
    }

    pub fn recv_range(&self) -> f64 {
        self.range
    }

    pub fn recv_status(&self) -> bool {
        self.status
    }
}
