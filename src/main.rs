extern crate space;

use legion::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use space::components::engines::Engines;
use space::components::navigation::Navigation;
use space::components::storage::Storage;
use space::constants;
use space::math::{rand_name, Vector};
use space::requests;

fn populate(world: &mut World) {
    for _ in 0..constants::ASTROID_COUNT {
        Astroid::insert_to(world);
    }
    Ship::insert_to(world);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Astroid;

impl Astroid {
    fn insert_to(world: &mut World) {
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
struct Ship;

impl Ship {
    fn insert_to(world: &mut World) {
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

#[derive(Debug, Clone, Default, PartialEq)]
struct Acceleration(Vector);

#[derive(Debug, Clone, Default, PartialEq)]
struct Velocity(Vector);

#[derive(Debug, Clone, Default, PartialEq)]
struct Position(Vector);

#[derive(Debug, Clone, Default, PartialEq)]
struct Name(String);

fn main() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    populate(&mut world);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        requests::run(tx).unwrap();
    });

    let mut requests = Vec::new();

    let mut run = true;
    while run {
        let timer = Instant::now();

        process(&mut world);

        if !requests.is_empty() {
            println!("{:?}", requests);
        }

        requests.clear();
        while timer.elapsed().as_millis() < constants::LOOP_DURATION_MS.into() {
            thread::sleep(Duration::from_millis(1));
            match rx.try_recv() {
                Ok(request) => requests.push(request),
                Err(mpsc::TryRecvError::Disconnected) => run = false,
                Err(mpsc::TryRecvError::Empty) => (),
            }
        }
    }
}

fn process(world: &mut World) {
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
