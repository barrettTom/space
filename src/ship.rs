use mass::Mass;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    name        : String,
    location    : (isize, isize, isize),
}

impl Mass for Ship {
    fn new(name : &str, location : (isize, isize, isize)) -> Ship {
        Ship {
            name        : String::from(name),
            location    : location,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_location(&self) -> (isize, isize, isize) {
        self.location
    }

    fn give_location(&mut self, location : (isize, isize, isize)) {
        self.location = location;
    }

    fn serialize(&self) ->String {
        serde_json::to_string(self).unwrap()
    }
}
