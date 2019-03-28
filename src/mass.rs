extern crate rand;

use self::rand::distributions::Uniform;
use self::rand::Rng;
use std::collections::HashMap;

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
use crate::schema::masses as db_masses;
use crate::storage::Storage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mass {
    pub mass_type: MassType,
    pub position: Vector,
    pub velocity: Vector,
    pub effects: Effects,
}

#[derive(Queryable, Insertable)]
#[table_name = "db_masses"]
pub struct MassEntry {
    pub id: Option<i32>,
    pub name: String,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub type_data: serde_json::Value,
}

impl MassEntry {
    pub fn to_mass(&self) -> Mass {
        Mass {
            position: Vector::new(self.pos_x, self.pos_y, self.pos_z),
            velocity: Vector::new(self.vel_x, self.vel_y, self.vel_z),
            mass_type: serde_json::from_str(&serde_json::to_string(&self.type_data).unwrap())
                .unwrap(),
            effects: Effects::new(),
        }
    }
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
        for _ in 0..rng.gen_range(
            constants::ASTROID_STARTING_MINERALS_MIN,
            constants::ASTROID_STARTING_MINERALS_MAX,
        ) {
            resources.push(Item::new(ItemType::CrudeMinerals));
        }

        let astroid = MassType::Astroid {
            resources: Storage::new(resources, constants::ASTROID_STORAGE_CAPACITY),
        };

        Mass {
            mass_type: astroid,
            position: Vector::new(
                rng.sample(p_range),
                rng.sample(p_range),
                rng.sample(p_range),
            ),
            velocity: Vector::new(
                rng.sample(v_range),
                rng.sample(v_range),
                rng.sample(v_range),
            ),
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

    pub fn process(&mut self, masses: &mut HashMap<String, Mass>) {
        if let MassType::Ship {
            ref mut navigation,
            ref mut engines,
            ref mut mining,
            ref mut refinery,
            ref mut construction,
            ref mut storage,
            ref mut tractorbeam,
            ..
        } = self.mass_type
        {
            if let Some(target_name) = &navigation.target_name {
                let mut target = masses.remove(target_name).unwrap();
                mining.process(self.position.clone(), masses, &mut target, storage);
                let acquired = tractorbeam.process(self.position.clone(), &mut target);

                if acquired {
                    if let MassType::Item { item } = target.mass_type {
                        storage.give_item(item);
                    }
                } else {
                    masses.insert(target_name.to_string(), target);
                }
            }

            let target = match &navigation.target_name {
                Some(target_name) => masses.get(target_name),
                None => None,
            };

            engines.process(self.position.clone(), self.velocity.clone(), target);
            refinery.process(storage);
            construction.process(
                self.velocity.clone(),
                self.position.clone(),
                masses,
                storage,
            );
            navigation.process(self.position.clone(), masses);
            self.effects.give_acceleration(engines.take_acceleration());
        }

        self.velocity += self.effects.take_acceleration();
        self.position += self.velocity.clone();
    }

    pub fn get_client_data(
        &self,
        module_type: ModuleType,
        masses: &HashMap<String, Mass>,
    ) -> String {
        if let MassType::Ship {
            ref navigation,
            ref engines,
            ref mining,
            ref refinery,
            ref construction,
            ref storage,
            ref tractorbeam,
            ..
        } = self.mass_type
        {
            let target = match &navigation.target_name {
                Some(target_name) => masses.get(target_name),
                None => None,
            };
            match module_type {
                ModuleType::Navigation => navigation.get_client_data(self.position.clone(), masses),
                ModuleType::Engines => engines.get_client_data(navigation.status.clone()),
                ModuleType::Mining => mining.get_client_data(self.position.clone(), target),
                ModuleType::Dashboard => serde_json::to_string(&self).unwrap() + "\n",
                ModuleType::Construction => construction.get_client_data(storage),
                ModuleType::Refinery => refinery.get_client_data(storage),
                ModuleType::Tractorbeam => tractorbeam.get_client_data(target),
            }
        } else {
            String::new()
        }
    }

    pub fn give_received_data(&mut self, module_type: ModuleType, recv: String) {
        if let MassType::Ship {
            ref mut navigation,
            ref mut engines,
            ref mut mining,
            ref mut refinery,
            ref mut construction,
            ref mut tractorbeam,
            ..
        } = self.mass_type
        {
            match module_type {
                ModuleType::Navigation => navigation.give_received_data(recv),
                ModuleType::Engines => engines.give_received_data(recv, self.velocity.clone()),
                ModuleType::Mining => mining.give_received_data(recv),
                ModuleType::Construction => construction.give_received_data(recv),
                ModuleType::Refinery => refinery.give_received_data(recv),
                ModuleType::Tractorbeam => tractorbeam.give_received_data(recv),
                ModuleType::Dashboard => (),
            }
        }
    }

    pub fn take_item(&mut self, item_type: ItemType) -> Option<Item> {
        match self.mass_type {
            MassType::Ship {
                ref mut storage, ..
            } => storage.take_item(item_type),
            MassType::Astroid {
                ref mut resources, ..
            } => resources.take_item(item_type),
            _ => None,
        }
    }

    pub fn give_item(&mut self, item: Item) -> bool {
        match self.mass_type {
            MassType::Ship {
                ref mut storage, ..
            } => storage.give_item(item),
            MassType::Astroid {
                ref mut resources, ..
            } => resources.give_item(item),
            _ => false,
        }
    }

    pub fn item_count(&self, item_type: ItemType) -> usize {
        match &self.mass_type {
            MassType::Ship { storage, .. } => storage.item_count(item_type),
            MassType::Astroid { resources, .. } => resources.item_count(item_type),
            _ => 0,
        }
    }

    pub fn to_mass_entry(&self, name: String) -> MassEntry {
        MassEntry {
            id: None,
            name,
            pos_x: self.position.x,
            pos_y: self.position.y,
            pos_z: self.position.z,
            vel_x: self.velocity.x,
            vel_y: self.velocity.y,
            vel_z: self.velocity.z,
            type_data: serde_json::from_str(&serde_json::to_string(&self.mass_type).unwrap())
                .unwrap(),
        }
    }
}
