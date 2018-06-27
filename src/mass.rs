extern crate rand;

use self::rand::distributions::Range;
use self::rand::distributions::Sample;

use item::Item;
use storage::Storage;
use modules::mining::Mining;
use modules::engines::Engines;
use modules::types::ModuleType;
use modules::refinery::Refinery;
use modules::dashboard::Dashboard;
use modules::navigation::Navigation;
use modules::construction::Construction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mass {
    pub mass_type   : MassType,
    pub position    : (f64, f64, f64),
    pub velocity    : (f64, f64, f64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MassType {
    Ship {
        storage     : Storage,
        mining      : Option<Mining>,
        engines     : Option<Engines>,
        refinery    : Option<Refinery>,
        dashboard   : Option<Dashboard>,
        navigation  : Option<Navigation>,
        construction: Option<Construction>,
    },
    Astroid {
        resources   : Storage,
    },
    Item {
        item : Item,
    },
    Station {
        module_type : ModuleType,
    },
}

impl Mass {
    pub fn new_astroid() -> Mass {
        let mut rng = rand::thread_rng();

        let mut pr = Range::new(-50.0, 50.0);
        let position = (pr.sample(&mut rng), pr.sample(&mut rng), pr.sample(&mut rng));

        let mut vr = Range::new(-0.5, 0.5);
        let velocity = (vr.sample(&mut rng), vr.sample(&mut rng), vr.sample(&mut rng));

        let mut rr = Range::new(0, 20);
        let mut resources = Vec::new();
        for _ in 0..rr.sample(&mut rng) {
            resources.push(Item::new("Mineral", 1));
        }

        let astroid = MassType::Astroid {
            resources  : Storage::new(resources),
        };

        Mass {
            mass_type   : astroid,
            position    : position,
            velocity    : velocity,
        }
    }

    pub fn new_ship() -> Mass {
        let ship = MassType::Ship {
            mining      : Some(Mining::new()),
            engines     : Some(Engines::new()),
            refinery    : Some(Refinery::new()),
            dashboard   : Some(Dashboard::new()),
            navigation  : Some(Navigation::new()),
            construction: Some(Construction::new()),
            storage     : Storage::new(Vec::new()),
        };

        Mass {
            mass_type   : ship,
            position    : (0.0, 0.0, 0.0),
            velocity    : (0.0, 0.0, 0.0),
        }
    }

    pub fn new_item(item : Item, position : (f64, f64, f64), velocity : (f64, f64, f64)) -> Mass {
        Mass {
            mass_type   : MassType::Item{item : item},
            position    : position,
            velocity    : velocity,
        }
    }

    pub fn new_station(module_type : ModuleType, position : (f64, f64, f64), velocity : (f64, f64, f64)) -> Mass {
        let mass_type = MassType::Station {
            module_type : module_type
        };

        Mass {
            mass_type   : mass_type,
            position    : position,
            velocity    : velocity,
        }
    }

    pub fn get_modules(&self) -> Vec<ModuleType> {
        let mut modules = Vec::new();
        modules.push(ModuleType::Mining);
        modules.push(ModuleType::Engines);
        modules.push(ModuleType::Refinery);
        modules.push(ModuleType::Dashboard);
        modules.push(ModuleType::Navigation);
        modules.push(ModuleType::Construction);
        modules
    }

    pub fn process(&mut self) {
        let mut acceleration = (0.0, 0.0, 0.0);
        match self.mass_type {
            MassType::Ship{ref mut navigation, ref mut engines, ref mut mining, ref mut refinery, ref mut construction, ..} => {
                mining.as_mut().unwrap().process();
                refinery.as_mut().unwrap().process();
                navigation.as_mut().unwrap().process();
                construction.as_mut().unwrap().process();
                acceleration = engines.as_mut().unwrap().recv_acceleration();
            },
            _ => (),
        }
        self.accelerate(acceleration);
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    pub fn accelerate(&mut self, acceleration : (f64, f64, f64)) {
        self.velocity.0 += acceleration.0;
        self.velocity.1 += acceleration.1;
        self.velocity.2 += acceleration.2;
    }

    pub fn has_minerals(&self) -> bool {
        match self.mass_type {
            MassType::Ship{ref storage, ..} => storage.has_minerals(),
            MassType::Astroid{ref resources, ..} => resources.has_minerals(),
            _ => false,
        }
    }

    pub fn refined_count(&self) -> usize {
        match self.mass_type {
            MassType::Ship{ref storage, ..} => storage.refined_count(),
            _ => 0,
        }
    }

    pub fn take(&mut self, name : &str) -> Option<Item> {
        match self.mass_type {
            MassType::Ship{ref mut storage, ..} => storage.take(name),
            MassType::Astroid{ref mut resources, ..} => resources.take(name),
            _ => None,
        }
    }

    pub fn give(&mut self, item : Item) -> bool {
        match self.mass_type {
            MassType::Ship{ref mut storage, ..} => storage.give(item),
            MassType::Astroid{ref mut resources, ..} => resources.give(item),
            _ => false,
        }
    }
}
