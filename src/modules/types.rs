#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModuleType {
    Navigation,
    Mining,
    Engines,
    Dashboard,
}
