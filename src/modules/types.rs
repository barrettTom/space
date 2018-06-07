#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModuleType {
    Mining,
    Engines,
    Refinery,
    Dashboard,
    Navigation,
}
