pub enum Module {
    Dashboard,
    Navigation,
    Engines,
}

pub fn from_primitive(num : isize) -> Module {
    match num {
        0 => Module::Dashboard,
        1 => Module::Navigation,
        2 => Module::Engines,
        _ => Module::Dashboard,
    }
}
