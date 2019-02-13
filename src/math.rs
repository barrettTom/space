extern crate rand;

use self::rand::distributions::Alphanumeric;
use self::rand::Rng;
use std::iter::repeat;

pub fn rand_name() -> String {
    repeat(())
        .map(|()| rand::thread_rng().sample(Alphanumeric))
        .take(8)
        .collect()
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(v: (f64, f64, f64)) -> Vector {
        Vector {
            x: v.0,
            y: v.1,
            z: v.2,
        }
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
