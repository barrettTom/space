#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    pub location : (isize, isize, isize)
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            location : (0,0,0)
        }
    }
}
