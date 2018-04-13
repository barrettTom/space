#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleType {
    Mining,
    Engines,
    Dashboard,
    Navigation,
}
