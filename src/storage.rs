use item::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    items       : Vec<Item>,
    carrying    : usize,
    capacity    : usize,
}

impl Storage {
    pub fn new(items : Vec<Item>) -> Storage {
        let mut carrying = 0;
        for item in items.iter() {
            carrying += item.size;
        }
        Storage {
            items       : items,
            capacity    : 10,
            carrying    : carrying,
        }
    }

    pub fn has_minerals(&self) -> bool {
        match self.items.iter().position(|item| item.is_mineral()) {
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

    pub fn give(&mut self, item : Item) -> bool {
        match self.capacity >= self.carrying + item.size {
            true => {
                self.carrying += item.size;
                self.items.push(item);
                true
            },
            false => false,
        }
    }
}
