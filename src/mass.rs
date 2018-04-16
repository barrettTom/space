extern crate rand;

use std::collections::HashMap;
use self::rand::distributions::Range;
use self::rand::distributions::Sample;

use storage::Storage;
use module::Module;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mass {
    pub mass_type   : MassType,
    pub position    : (f64, f64, f64),
    pub velocity    : (f64, f64, f64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MassType {
    Ship {
        modules     : HashMap<String, Module>,
        storage     : Storage,
    },
    Astroid{
        resources   : Storage,
    },
}

impl Mass {
    pub fn new_astroid() -> Mass {
        let mut rng = rand::thread_rng();

        let mut pr = Range::new(-50.0, 50.0);
        let position = (pr.sample(&mut rng), pr.sample(&mut rng), pr.sample(&mut rng));

        let mut vr = Range::new(-0.5, 0.5);
        let velocity = (vr.sample(&mut rng), vr.sample(&mut rng), vr.sample(&mut rng));

        let astroid = MassType::Astroid {
            resources  : Storage::new(Vec::new()),
        };

        Mass {
            mass_type   : astroid,
            position    : position,
            velocity    : velocity,
        }
    }

    pub fn new_ship() -> Mass {
        let mut modules = HashMap::new();

        modules.insert("Mining".to_string(), Module::new_mining());
        modules.insert("Engines".to_string(), Module::new_engines());
        modules.insert("Dashboard".to_string(), Module::new_dashboard());
        modules.insert("Navigation".to_string(), Module::new_navigation());

        let ship = MassType::Ship {
            modules     : modules,
            storage     : Storage::new(Vec::new()),
        };

        Mass {
            mass_type   : ship,
            position    : (0.0, 0.0, 0.0),
            velocity    : (0.0, 0.0, 0.0),
        }
    }

    pub fn process(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
        match self.mass_type {
            MassType::Ship{ref mut modules, ..} => {
                for module in modules.values_mut() {
                    module.process();
                }
            },
            _ => (),
        }
    }

    pub fn accelerate(&mut self, acceleration : (f64, f64, f64)) {
        self.velocity.0 += acceleration.0;
        self.velocity.1 += acceleration.1;
        self.velocity.2 += acceleration.2;
    }
}
