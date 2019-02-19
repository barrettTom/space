use crate::item::{Item, ItemType};

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

    pub fn take_items(&mut self, item_type: ItemType, count: usize) -> Option<Vec<Item>> {
        if self
            .items
            .iter()
            .filter(|item| item.item_type == item_type)
            .count()
            >= count
        {
            let mut items = Vec::new();
            for _ in 0..count {
                let index = self
                    .items
                    .iter()
                    .position(|item| item.item_type == item_type)
                    .unwrap();
                let item = self.items.remove(index);
                self.carrying -= item.size;
                items.push(item);
            }
            Some(items)
        } else {
            None
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
}
