#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name : String,
    size : usize,
}

impl Item {
    pub fn new(name : &str, size : usize) -> Item {
        Item {
            name : String::from(name),
            size : size,
        }
    }
}
