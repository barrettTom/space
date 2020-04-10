extern crate space;

use legion::prelude::*;
use space::components::engines::Engines;
use space::components::navigation::Navigation;
use space::components::storage::Storage;
use space::math::{rand_name, Vector};
use std::net::TcpListener;
use std::thread::sleep;
use std::time::{Duration, Instant};

use space::constants;
use space::server_connection::ServerConnection;

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
    let listener = TcpListener::bind("localhost:6000").unwrap();
    listener.set_nonblocking(true).unwrap();

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
    let mut connections: Vec<ServerConnection> = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let new_connection = ServerConnection::new(stream);
                let exists = connections
                    .iter()
                    .position(|connection| connection.name == new_connection.name);
                if let Some(index) = exists {
                    connections.remove(index);
                }
                connections.push(new_connection);
            }
            _ => {
                let timer = Instant::now();

                for connection in &mut connections {
                    if connection.open {
                        /*
                        let mut ship = masses.remove(&connection.name).unwrap();

                        let send = ship.get_client_data(connection.module_type.clone(), &masses);
                        connection.open = connection.stream.write(send.as_bytes()).is_ok();

                        let recv = connection.receive();
                        ship.give_received_data(connection.module_type.clone(), recv);

                        masses.insert(connection.name.clone(), ship);
                        */
                    }
                }

                /*
                if backup_countdown == 0 {
                    let masses_clone = masses.clone();
                    spawn(move || backup(masses_clone));
                    backup_countdown = constants::BACKUP_COUNTDOWN;
                }
                */

                process(&mut world);

                if timer.elapsed().as_millis() < constants::LOOP_DURATION_MS.into() {
                    sleep(Duration::from_millis(
                        constants::LOOP_DURATION_MS - timer.elapsed().as_millis() as u64,
                    ));
                }
                //backup_countdown -= 1;
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
