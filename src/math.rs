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
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Vector {
    pub fn new(v: (f64, f64, f64)) -> Vector {
        Vector {
            a: v.0,
            b: v.1,
            c: v.2,
        }
    }

    pub fn distance_from(&self, other: Vector) -> f64 {
        ((self.a - other.a).powf(2.0) + (self.b - other.b).powf(2.0) + (self.c - other.c).powf(2.0))
            .sqrt()
    }

    pub fn unitize(&self) -> Vector {
        let denominator = self.magnitude();
        Vector {
            a: self.a / denominator,
            b: self.b / denominator,
            c: self.c / denominator,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.a.powf(2.0) + self.b.powf(2.0) + self.c.powf(2.0)).sqrt()
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.a, self.b, self.c)
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            a: self.a * other,
            b: self.b * other,
            c: self.c * other,
        }
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = Vector {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
        }
    }
}
