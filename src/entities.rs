use legion::prelude::*;

use crate::components::engines::Engines;
use crate::components::misc::{Name, Position, Velocity};
use crate::components::navigation::Navigation;
use crate::components::storage::Storage;
use crate::constants;
use crate::math::rand_name;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Astroid;

impl Astroid {
    pub fn insert_to(world: &mut World) {
        world.insert(
            (Astroid, true),
            vec![(
                Name(rand_name()),
                Velocity::default(),
                Position::default(),
                Storage::new(Vec::new(), constants::ASTROID_STORAGE_CAPACITY),
            )],
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ship;

impl Ship {
    pub fn insert_to(world: &mut World) {
        world.insert(
            (Ship, true),
            vec![(
                Name(rand_name()),
                Velocity::default(),
                Position::default(),
                Storage::new(Vec::new(), constants::SHIP_STORAGE_CAPACITY),
                Engines::new(),
                Navigation::new(),
            )],
        );
    }
}
