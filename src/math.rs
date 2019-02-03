extern crate rand;

use self::rand::distributions::Alphanumeric;
use self::rand::Rng;
use std::iter::repeat;

pub fn distance(l0: (f64, f64, f64), l1: (f64, f64, f64)) -> f64 {
    ((l1.0 - l0.0).powf(2.0) + (l1.1 - l0.1).powf(2.0) + (l1.2 - l0.2).powf(2.0)).sqrt()
}

pub fn rand_name() -> String {
    repeat(())
        .map(|()| rand::thread_rng().sample(Alphanumeric))
        .take(8)
        .collect()
}
