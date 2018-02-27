use mass::{Mass, Type};
extern crate rand;
use self::rand::distributions::Range;
use astroid::rand::distributions::Sample;
extern crate serde_json;
use astroid::rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Astroid {
    name        : String,
    t           : Type,
    position    : (f64, f64, f64),
    velocity    : (f64, f64, f64),
}

impl Astroid {
    pub fn new() -> Astroid {
        let name : String = rand::thread_rng()
                            .gen_ascii_chars()
                            .take(8)
                            .collect();
        let mut r = Range::new(-50.0, 50.0);
        let mut rng = rand::thread_rng();
        let position = (r.sample(&mut rng), r.sample(&mut rng), r.sample(&mut rng));
        Astroid {
            name        : name,
            t           : Type::Astroid,
            position    : position,
            velocity    : (0.0, 0.0, 0.0),
        }
    }
}

impl Mass for Astroid {
    fn name(&self) -> &String {
        &self.name
    }

    fn position(&self) -> (f64, f64, f64) {
        self.position
    }

    fn serialize(&self) ->String {
        serde_json::to_string(self).unwrap()
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
