use mass::Mass;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Astroid {
    name        : String,
    location    : (isize, isize, isize),
}

impl Mass for Astroid {
    fn new(name : &str, location : (isize, isize, isize)) -> Astroid {
        Astroid {
            name : String::from(name),
            location : location,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn location(&self) -> (isize, isize, isize) {
        self.location
    }

    fn set_location(&mut self, location : (isize, isize, isize)) {
        self.location = location;
    }

    fn serialize(&self) ->String {
        serde_json::to_string(self).unwrap()
    }

    fn deserialize(&mut self, data : &str) {
    }
}
