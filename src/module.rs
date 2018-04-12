use downcast::Any;

pub trait Module : Any {
    fn box_clone(&self) -> Box<Module>;
}

impl Clone for Box<Module> {
    fn clone(&self) -> Box<Module> {
        self.box_clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleType {
    Mining,
    Engines,
    Dashboard,
    Navigation,
}

downcast!(Module);
