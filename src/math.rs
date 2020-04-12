extern crate rand;

use self::rand::distributions::Alphanumeric;
use self::rand::Rng;
use std::iter::repeat;

use crate::components::types::ModuleType;
use crate::constants;

pub fn rand_name() -> String {
    repeat(())
        .map(|()| rand::thread_rng().sample(Alphanumeric))
        .take(8)
        .collect()
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ControlSystem {
    previous_error: f64,
    integral: f64,
    kp: f64,
    ki: f64,
    kd: f64,
    dt: f64,
}

impl ControlSystem {
    pub fn new(module: ModuleType) -> ControlSystem {
        let previous_error = 0.0;
        let integral = 0.0;
        match module {
            ModuleType::Tractorbeam => ControlSystem {
                kp: constants::SHIP_TRACTORBEAM_CONTROLSYSTEM_KP,
                ki: constants::SHIP_TRACTORBEAM_CONTROLSYSTEM_KI,
                kd: constants::SHIP_TRACTORBEAM_CONTROLSYSTEM_KD,
                dt: constants::SHIP_TRACTORBEAM_CONTROLSYSTEM_DT,
                previous_error,
                integral,
            },
            _ => ControlSystem {
                kp: 1.0,
                ki: 1.0,
                kd: 1.0,
                dt: 1.0,
                previous_error,
                integral,
            },
        }
    }

    pub fn compute(&mut self, strength: f64, distance: f64, desired_distance: f64) -> f64 {
        let error = desired_distance - distance;
        self.integral += error * self.dt;
        let derivative = (error - self.previous_error) / self.dt;
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.previous_error = error;

        if output.abs() > strength {
            if output.is_sign_positive() {
                strength
            } else {
                strength * -1.0
            }
        } else {
            output
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn distance_from(&self, other: Vector) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0))
            .sqrt()
    }

    pub fn unitize(&self) -> Vector {
        let denominator = self.magnitude();
        Vector {
            x: self.x / denominator,
            y: self.y / denominator,
            z: self.z / denominator,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
