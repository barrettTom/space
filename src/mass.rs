extern crate rand;

use self::rand::distributions::Range;
use self::rand::distributions::Sample;

use storage::Storage;
use module::ModuleType;
use targeting::Targeting;
use mining::Mining;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mass {
    pub mass_type   : MassType,
    pub position    : (f64, f64, f64),
    pub velocity    : (f64, f64, f64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MassType {
    Ship {
        modules     : Vec<ModuleType>,
        mining      : Mining,
        targeting   : Targeting,
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
        let mut modules = Vec::new();

        modules.push(ModuleType::Navigation);
        modules.push(ModuleType::Engines);
        modules.push(ModuleType::Dashboard);
        modules.push(ModuleType::Mining);

        let ship = MassType::Ship {
            modules     : modules,
            mining      : Mining::new(),
            targeting   : Targeting::new(),
            storage     : Storage::new(Vec::new()),
        };

        Mass {
            mass_type   : ship,
            position    : (0.0,0.0,0.0),
            velocity    : (0.0,0.0,0.0),
        }
    }

    pub fn process(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
        match self.mass_type {
            MassType::Ship{ref mut targeting, ..} => {
                targeting.process();
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
