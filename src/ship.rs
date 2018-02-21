#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    pub name        : String,
    pub location    : (isize, isize, isize),
}

impl Ship {
    pub fn new(name : &str) -> Ship {
        Ship {
            name        : String::from(name),
            location    : (0,0,0),
        }
    }
}
