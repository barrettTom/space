use crate::components::item::{Item, ItemType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub items: Vec<Item>,
    carrying: usize,
    capacity: usize,
}

impl Storage {
    pub fn new(items: Vec<Item>, capacity: usize) -> Storage {
        let mut carrying = 0;
        for item in items.iter() {
            carrying += item.size;
        }
        Storage {
            items,
            capacity,
            carrying,
        }
    }

    pub fn take_item(&mut self, item_type: ItemType) -> Option<Item> {
        match self
            .items
            .iter()
            .position(|item| item.item_type == item_type)
        {
            Some(index) => {
                let item = self.items.remove(index);
                self.carrying -= item.size;
                Some(item)
            }
            None => None,
        }
    }

    pub fn give_item(&mut self, item: Item) -> bool {
        if self.capacity >= self.carrying + item.size {
            self.carrying += item.size;
            self.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn item_count(&self, item_type: ItemType) -> usize {
        self.items
            .iter()
            .filter(|item| item.item_type == item_type)
            .count()
    }
}
