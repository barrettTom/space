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
    pub itemtype: ItemType,
    pub name: String,
    pub size: usize,
}

impl Item {
    pub fn new(itemtype: ItemType) -> Item {
        let size = match itemtype {
            ItemType::Iron => constants::IRON_SIZE,
            ItemType::Hydrogen => constants::HYDROGEN_SIZE,
            ItemType::CrudeMinerals => constants::CRUDE_MINERALS_SIZE,
        };
        Item {
            name: serde_json::to_string(&itemtype).unwrap() + &rand_name(),
            itemtype,
            size,
        }
    }
}
