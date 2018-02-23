use mass::Mass;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    name        : String,
    location    : (f64, f64, f64),
    r           : f64,
}

impl Ship {
    pub fn range(&self) -> f64 {
        self.r
    }
}

impl Mass for Ship {
    fn new(name : &str, location : (f64, f64, f64)) -> Ship {
        Ship {
            name        : String::from(name),
            location    : location,
            r           : 100.0,
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

    fn deserialize(&mut self, data : &str) {
        //self = serde_json::from_str(data).unwrap();
    }
}
