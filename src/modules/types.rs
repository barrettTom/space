use self::ModuleType::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModuleType {
    Mining,
    Engines,
    Refinery,
    Dashboard,
    Navigation,
    Tractorbeam,
    Construction,
}

impl ModuleType {
    pub fn iter() -> Vec<ModuleType> {
        let mut vec = Vec::new();
        vec.push(Mining);
        vec.push(Engines);
        vec.push(Refinery);
        vec.push(Dashboard);
        vec.push(Navigation);
        vec.push(Tractorbeam);
        vec.push(Construction);
        vec
    }
}
