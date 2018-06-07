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

    pub fn has_minerals(&self) -> bool {
        match self.items.iter().position(|item| item.name == "Iron") {
            Some(_) => true,
            None => false,
        }
    }

    pub fn take(&mut self, name : &str) -> Option<Item> {
        match self.items.iter().position(|item| item.name == name) {
            Some(index) => Some(self.items.remove(index)),
            None => None,
        }
    }

    pub fn give(&mut self, item : Item) {
        self.items.push(item);
    }
}
