use mass::{Mass, Type};
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    name        : String,
    position    : (f64, f64, f64),
    velocity    : (f64, f64, f64),
    mass_type   : Type,
    r           : f64,
    target      : Option<usize>,
}

impl Ship {
    pub fn new(name : &str, position : (f64, f64, f64)) -> Ship {
        Ship {
            name        : String::from(name),
            position    : position,
            velocity    : (0.0, 0.0, 0.0),
            mass_type   : Type::Ship,
            r           : 100.0,
            target      : None,
        }
    }

    pub fn range(&self) -> f64 {
        self.r
    }
}

impl Mass for Ship {
    fn name(&self) -> &String {
        &self.name
    }

    fn position(&self) -> (f64, f64, f64) {
        self.position
    }

    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn slow(&mut self) {
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

    fn process(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn give_acceleration(&mut self, acceleration : (f64, f64, f64)) {
        self.velocity.0 += acceleration.0;
        self.velocity.1 += acceleration.1;
        self.velocity.2 += acceleration.2;
    }
}
