pub enum Module {
    Dashboard,
    Navigation,
    Engines,
}

pub fn from_primitive(data : String) -> Module {
    let data = data.replace("\n", "");
    let num = data.parse::<isize>().unwrap();

    match num {
        0 => Module::Dashboard,
        1 => Module::Navigation,
        2 => Module::Engines,
        _ => Module::Dashboard,
    }
}
