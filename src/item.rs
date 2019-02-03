#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub size: usize,
}

impl Item {
    pub fn new(name: &str, size: usize) -> Item {
        Item {
            name: String::from(name),
            size,
        }
    }

    pub fn is_mineral(&self) -> bool {
        self.name == "Mineral"
    }
}
