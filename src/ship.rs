extern crate serde_json;

use module::Module;
use mass::{Mass, Type};
use targeting::{Targeting, TargetingStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    name        : String,
    position    : (f64, f64, f64),
    velocity    : (f64, f64, f64),
    mass_type   : Type,
    r           : f64,
    modules     : Vec<Module>,
    targeting   : Targeting,
}

impl Ship {
    pub fn new(name : &str, position : (f64, f64, f64)) -> Ship {
        let mut modules = Vec::new();

        modules.push(Module::Navigation);
        modules.push(Module::Engines);
        modules.push(Module::Dashboard);

        Ship {
            name            : String::from(name),
            position        : position,
            velocity        : (0.0, 0.0, 0.0),
            mass_type       : Type::Ship,
            r               : 100.0,
            targeting       : Targeting::new(),
            modules         : modules,
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

    pub fn range(&self) -> f64 {
        self.r
    }

    pub fn give_target(&mut self, target : Option<String>) {
        self.targeting.give_target(target);
    }

    pub fn recv_target(&self) -> Option<String> {
        self.targeting.get_target()
    }

    pub fn recv_target_status(&self) -> TargetingStatus {
        self.targeting.get_status()
    }

    pub fn get_modules(&self) -> String {
        serde_json::to_string(&self.modules).unwrap() + "\n"
    }
}

impl Mass for Ship {
    fn name(&self) -> &String {
        &self.name
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
