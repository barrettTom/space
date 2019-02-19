use crate::constants;
use crate::math::rand_name;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ItemType {
    CrudeMinerals,
    Iron,
    Hydrogen,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
    pub name: String,
    pub size: usize,
}

impl Item {
    pub fn new(item_type: ItemType) -> Item {
        let size = match item_type {
            ItemType::Iron => constants::IRON_SIZE,
            ItemType::Hydrogen => constants::HYDROGEN_SIZE,
            ItemType::CrudeMinerals => constants::CRUDE_MINERALS_SIZE,
        };
        Item {
            name: serde_json::to_string(&item_type).unwrap() + &rand_name(),
            item_type,
            size,
        }
    }
}
