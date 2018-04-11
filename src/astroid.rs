use self::rand::distributions::Range;
use astroid::rand::distributions::Sample;

extern crate rand;
extern crate serde_json;

use storage::Storage;
use astroid::rand::Rng;
use mass::{Mass, MassType};
use item::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Astroid {
    name        : String,
    mass_type   : MassType,
    position    : (f64, f64, f64),
    velocity    : (f64, f64, f64),
    resouces    : Storage,
}

impl Astroid {
    pub fn new() -> Astroid {
        let name : String = rand::thread_rng()
                            .gen_ascii_chars()
                            .take(8)
                            .collect();
        let mut rng = rand::thread_rng();

        let mut pr = Range::new(-50.0, 50.0);
        let position = (pr.sample(&mut rng), pr.sample(&mut rng), pr.sample(&mut rng));

        let mut vr = Range::new(-0.5, 0.5);
        let velocity = (vr.sample(&mut rng), vr.sample(&mut rng), vr.sample(&mut rng));

        let mut rr = Range::new(0, 20);
        let mut resouces = Vec::new();
        for _ in 0..rr.sample(&mut rng) {
            resouces.push(Item::new("Iron", 1))
        }
        Astroid {
            name        : name,
            mass_type   : MassType::Astroid,
            position    : position,
            velocity    : velocity,
            resouces    : Storage::new(resouces),
        }
    }
}

impl Mass for Astroid {
    fn name(&self) -> &String {
        &self.name
    }

    fn recv_mass_type(&self) -> MassType {
        self.mass_type.clone()
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

    fn process(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
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
