extern crate space;

use legion::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use space::constants;
use space::math::Vector;
use space::modules::engines::Engines;
use space::modules::navigation::Navigation;
use space::requests;
use space::storage::Storage;

fn populate(world: &mut World) {
    for _ in 0..constants::ASTROID_COUNT {
        Astroid::insert_to(world);
    }
    Ship::insert_to(world);
}

/*
fn backup(_masses: HashMap<String, Mass>) {
    let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");
    let timestamp = SystemTime::now();
    for (name, mass) in masses {
        let mass_entry = mass.to_mass_entry(name.to_string(), timestamp);
        diesel::insert_into(db_masses)
            .values(&mass_entry)
            .on_conflict(name_column)
            .do_update()
            .set(&mass_entry)
            .execute(&connection)
            .expect("Cannot backup");
    }
}

fn restore() -> HashMap<String, Mass> {
    let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");
    db_masses
        .load::<MassEntry>(&connection)
        .expect("Cannot query, are you sure you can restore?")
        .iter()
        .map(|mass_entry| mass_entry.to_mass())
        .collect()
}
    */

#[derive(Debug, Clone, Copy, PartialEq)]
struct Astroid;

impl Astroid {
    fn insert_to(world: &mut World) {
        world.insert(
            (Astroid, true),
            vec![(
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

fn main() {
    /*
    let matches = App::new("space server")
        .subcommand(SubCommand::with_name("--restore"))
        .get_matches();
    */

    let universe = Universe::new();
    let mut world = universe.create_world();

    populate(&mut world);

    //let mut masses = match matches.subcommand_name() {
    //    Some("--restore") => restore(),
    //    _ => populate(),
    //};

    //let mut backup_countdown = constants::BACKUP_COUNTDOWN;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        requests::run(tx).unwrap();
    });

    loop {
        let timer = Instant::now();

        let mut requests = Vec::new();
        while let request = rx.recv() {
            requests.push(request);
        }
        process(&mut world);

        /*
        if backup_countdown == 0 {
            let masses_clone = masses.clone();
            spawn(move || backup(masses_clone));
            backup_countdown = constants::BACKUP_COUNTDOWN;
        }
        backup_countdown -= 1;
        */

        if timer.elapsed().as_millis() < constants::LOOP_DURATION_MS.into() {
            thread::sleep(Duration::from_millis(
                constants::LOOP_DURATION_MS - timer.elapsed().as_millis() as u64,
            ));
        }
    }
}

fn process(world: &mut World) {
    for (mut engines, navigation, position, velocity) in <(
        Write<Engines>,
        Read<Navigation>,
        Read<Position>,
        Read<Velocity>,
    )>::query()
    .iter(world)
    {
        engines.process(position.0, velocity.0, None);
    }

    for (mut velocity, acceleration) in <(Write<Velocity>, Read<Acceleration>)>::query().iter(world)
    {
        velocity.0 += acceleration.0;
    }

    for (mut position, velocity) in <(Write<Position>, Read<Velocity>)>::query().iter(world) {
        position.0 += velocity.0;
    }
}
