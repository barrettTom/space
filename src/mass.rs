extern crate rand;

use self::rand::distributions::Uniform;
use self::rand::Rng;

use crate::constants;
use crate::item::{Item, ItemType};
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
        mining: Mining,
        engines: Engines,
        refinery: Refinery,
        dashboard: Dashboard,
        navigation: Navigation,
        tractorbeam: Tractorbeam,
        construction: Construction,
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

        let p_range = Uniform::new(
            constants::ASTROID_STARTING_POSITION_MAX * -1.0,
            constants::ASTROID_STARTING_POSITION_MAX,
        );
        let v_range = Uniform::new(
            constants::ASTROID_STARTING_VELOCITY_MAX * -1.0,
            constants::ASTROID_STARTING_VELOCITY_MAX,
        );

        let mut resources = Vec::new();
        for _ in 0..rng.gen_range(0, constants::ASTROID_STARTING_MINERALS_MAX) {
            resources.push(Item::new(ItemType::CrudeMinerals));
        }

        let astroid = MassType::Astroid {
            resources: Storage::new(resources, constants::ASTROID_STORAGE_CAPACITY),
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
            mining: Mining::new(),
            engines: Engines::new(),
            refinery: Refinery::new(),
            dashboard: Dashboard::new(),
            navigation: Navigation::new(),
            tractorbeam: Tractorbeam::new(),
            construction: Construction::new(),
            storage: Storage::new(Vec::new(), constants::SHIP_STORAGE_CAPACITY),
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
            mining.process();
            refinery.process();
            navigation.process();
            construction.process();
            engines.process(self.velocity.clone());
            self.effects.give_acceleration(engines.take_acceleration())
        }

        self.velocity += self.effects.take_acceleration();
        self.position += self.velocity.clone();
    }
}
