extern crate rand;

use self::rand::distributions::Uniform;
use self::rand::Rng;

use crate::item::Item;
use crate::math::Vector;
use crate::modules::construction::Construction;
use crate::modules::dashboard::Dashboard;
use crate::modules::engines::Engines;
use crate::modules::mining::Mining;
use crate::modules::navigation::Navigation;
use crate::modules::refinery::Refinery;
use crate::modules::tractorbeam::Tractorbeam;
use crate::modules::types::ModuleType;
use crate::storage::Storage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mass {
    pub mass_type: MassType,
    pub position: Vector,
    pub velocity: Vector,
    pub effects: Effects,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Effects {
    acceleration: Vector,
}

impl Effects {
    pub fn new() -> Effects {
        Effects {
            acceleration: Vector::default(),
        }
    }

    pub fn give_acceleration(&mut self, acceleration: Vector) {
        self.acceleration += acceleration;
    }

    pub fn take_acceleration(&mut self) -> Vector {
        let acceleration = self.acceleration.clone();
        self.acceleration = Vector::default();
        acceleration
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MassType {
    Ship {
        storage: Storage,
        mining: Option<Mining>,
        engines: Option<Engines>,
        refinery: Option<Refinery>,
        dashboard: Option<Dashboard>,
        navigation: Option<Navigation>,
        tractorbeam: Option<Tractorbeam>,
        construction: Option<Construction>,
    },
    Astroid {
        resources: Storage,
    },
    Item {
        item: Item,
    },
    Station {
        module_type: ModuleType,
    },
}

impl Mass {
    pub fn new_astroid() -> Mass {
        let mut rng = rand::thread_rng();

        let p_range = Uniform::new(-50.0, 50.0);
        let v_range = Uniform::new(-0.5, 0.5);

        let mut resources = Vec::new();
        for _ in 0..rng.gen_range(0, 20) {
            resources.push(Item::new("Mineral", 1));
        }

        let astroid = MassType::Astroid {
            resources: Storage::new(resources),
        };

        Mass {
            mass_type: astroid,
            position: Vector::new((
                rng.sample(p_range),
                rng.sample(p_range),
                rng.sample(p_range),
            )),
            velocity: Vector::new((
                rng.sample(v_range),
                rng.sample(v_range),
                rng.sample(v_range),
            )),
            effects: Effects::new(),
        }
    }

    pub fn new_ship() -> Mass {
        let ship = MassType::Ship {
            mining: Some(Mining::new()),
            engines: Some(Engines::new()),
            refinery: Some(Refinery::new()),
            dashboard: Some(Dashboard::new()),
            navigation: Some(Navigation::new()),
            tractorbeam: Some(Tractorbeam::new()),
            construction: Some(Construction::new()),
            storage: Storage::new(Vec::new()),
        };

        Mass {
            mass_type: ship,
            position: Vector::default(),
            velocity: Vector::default(),
            effects: Effects::new(),
        }
    }

    pub fn new_item(item: Item, position: Vector, velocity: Vector) -> Mass {
        Mass {
            mass_type: MassType::Item { item },
            position,
            velocity,
            effects: Effects::new(),
        }
    }

    pub fn new_station(module_type: ModuleType, position: Vector, velocity: Vector) -> Mass {
        let mass_type = MassType::Station { module_type };

        Mass {
            mass_type,
            position,
            velocity,
            effects: Effects::new(),
        }
    }

    pub fn get_modules(&self) -> Vec<ModuleType> {
        let mut modules = Vec::new();
        modules.push(ModuleType::Mining);
        modules.push(ModuleType::Engines);
        modules.push(ModuleType::Refinery);
        modules.push(ModuleType::Dashboard);
        modules.push(ModuleType::Navigation);
        modules.push(ModuleType::Tractorbeam);
        modules.push(ModuleType::Construction);
        modules
    }

    pub fn process(&mut self) {
        if let MassType::Ship {
            ref mut navigation,
            ref mut engines,
            ref mut mining,
            ref mut refinery,
            ref mut construction,
            ..
        } = self.mass_type
        {
            mining.as_mut().unwrap().process();
            refinery.as_mut().unwrap().process();
            navigation.as_mut().unwrap().process();
            construction.as_mut().unwrap().process();
            self.effects
                .give_acceleration(engines.as_mut().unwrap().recv_acceleration())
        }

        self.velocity += self.effects.take_acceleration();
        self.position += self.velocity.clone();
    }

    pub fn has_minerals(&self) -> bool {
        match self.mass_type {
            MassType::Ship { ref storage, .. } => storage.has_minerals(),
            MassType::Astroid { ref resources, .. } => resources.has_minerals(),
            _ => false,
        }
    }

    pub fn refined_count(&self) -> usize {
        match self.mass_type {
            MassType::Ship { ref storage, .. } => storage.refined_count(),
            _ => 0,
        }
    }

    pub fn take(&mut self, name: &str) -> Option<Item> {
        match self.mass_type {
            MassType::Ship {
                ref mut storage, ..
            } => storage.take(name),
            MassType::Astroid {
                ref mut resources, ..
            } => resources.take(name),
            _ => None,
        }
    }

    pub fn give(&mut self, item: Item) -> bool {
        match self.mass_type {
            MassType::Ship {
                ref mut storage, ..
            } => storage.give(item),
            MassType::Astroid {
                ref mut resources, ..
            } => resources.give(item),
            _ => false,
        }
    }
}
