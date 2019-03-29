pub const ASTROID_COUNT: usize = 10;
pub const ASTROID_STORAGE_CAPACITY: usize = 100;
pub const ASTROID_STARTING_MINERALS_MAX: usize = 20;
pub const ASTROID_STARTING_MINERALS_MIN: usize = 5;
pub const ASTROID_STARTING_VELOCITY_MAX: f64 = 0.5;
pub const ASTROID_STARTING_POSITION_MAX: f64 = 50.0;

pub const SHIP_STORAGE_CAPACITY: usize = 100;
pub const SHIP_CONSTRUCTION_IRON_COST: usize = 5;
pub const SHIP_CONSTRUCTION_TIME: u64 = 5;
pub const SHIP_MINING_TIME: u64 = 5;
pub const SHIP_MINING_RANGE: f64 = 10.0;
pub const SHIP_NAVIGATION_TIME: u64 = 3;
pub const SHIP_NAVIGATION_RANGE: f64 = 100.0;
pub const SHIP_REFINERY_TIME: u64 = 5;
pub const SHIP_TRACTORBEAM_STRENGTH: f64 = 0.1;
pub const SHIP_TRACTORBEAM_RANGE: f64 = 50.0;
pub const SHIP_TRACTORBEAM_ACQUIRE_RANGE: f64 = 1.0;
pub const SHIP_TRACTORBEAM_CONTROLSYSTEM_KP: f64 = 1.0;
pub const SHIP_TRACTORBEAM_CONTROLSYSTEM_KI: f64 = 0.01;
pub const SHIP_TRACTORBEAM_CONTROLSYSTEM_KD: f64 = 0.001;
pub const SHIP_TRACTORBEAM_CONTROLSYSTEM_DT: f64 = 0.0001;
pub const SHIP_ENGINES_FUEL_START: f64 = 100.0;
pub const SHIP_ENGINES_ACCELERATION: f64 = 0.1;

pub const IRON_SIZE: usize = 1;
pub const HYDROGEN_SIZE: usize = 1;
pub const CRUDE_MINERALS_SIZE: usize = 10;

pub const FLOAT_PRECISION: f64 = 0.001;
pub const LOOP_DURATION_MS: u64 = 100;

pub const POSTGRES_USERNAME: &str = "space";
pub const POSTGRES_PASSWORD: &str = "space";
pub const POSTGRES_IP: &str = "localhost";
pub const POSTGRES_DB_NAME: &str = "space_db";
