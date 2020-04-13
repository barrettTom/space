use legion::prelude::*;

use crate::components::engines::Engines;
use crate::components::misc::{Acceleration, Position, Velocity};
use crate::components::navigation::Navigation;
//use crate::components::storage::Storage;

pub fn process(world: &mut World) {
    for (mut engines, mut acceleration, navigation, position, velocity) in <(
        Write<Engines>,
        Write<Acceleration>,
        Read<Navigation>,
        Read<Position>,
        Read<Velocity>,
    )>::query()
    .iter(world)
    {
        let (target_position, target_velocity) = match &navigation.target {
            Some(target) => (Some(target.position), Some(target.velocity)),
            None => (None, None),
        };
        engines.process(
            position.0,
            velocity.0,
            &mut acceleration.0,
            target_position,
            target_velocity,
        );
    }

    for (mut velocity, acceleration) in <(Write<Velocity>, Read<Acceleration>)>::query().iter(world)
    {
        velocity.0 += acceleration.0;
    }

    for (mut position, velocity) in <(Write<Position>, Read<Velocity>)>::query().iter(world) {
        position.0 += velocity.0;
    }
}
