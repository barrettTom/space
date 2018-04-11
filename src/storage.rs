use item::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    items       : Vec<Item>,
    capacity    : usize,
}

impl Storage {
    pub fn new(items : Vec<Item>) -> Storage {
        Storage {
            items       : items,
            capacity    : 100,
        }
    }
}
