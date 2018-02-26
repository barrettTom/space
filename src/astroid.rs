use mass::{Mass, Type};
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Astroid {
    name        : String,
    t           : Type,
    location    : (f64, f64, f64),
}

impl Mass for Astroid {
    fn new(name : &str, location : (f64, f64, f64)) -> Astroid {
        Astroid {
            name : String::from(name),
            t : Type::Astroid,
            location : location,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn location(&self) -> (f64, f64, f64) {
        self.location
    }

    fn set_location(&mut self, location : (f64, f64, f64)) {
        self.location = location;
    }

    fn serialize(&self) ->String {
        serde_json::to_string(self).unwrap()
    }
}
