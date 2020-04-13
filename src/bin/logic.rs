extern crate space;

use diesel::prelude::*;
use legion::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

use space::constants;
use space::entities;
use space::request::Request;
use space::schema::requests::dsl;
use space::systems;

fn populate(world: &mut World) {
    for _ in 0..constants::ASTROID_COUNT {
        entities::Astroid::insert_to(world);
    }
    entities::Ship::insert_to(world);
}

fn process_requests(connection: &SqliteConnection) {
    let requests = dsl::requests
        .filter(dsl::received.eq(false))
        .load::<Request>(connection)
        .unwrap();

    if !requests.is_empty() {
        println!("{:?}", requests);
    }

    for request in requests.iter() {
        diesel::update(request)
            .set(dsl::received.eq(true))
            .execute(connection)
            .unwrap();
    }
}

fn main() {
    let connection = SqliteConnection::establish(constants::DB_PATH).unwrap();

    let universe = Universe::new();
    let mut world = universe.create_world();

    populate(&mut world);

    loop {
        let timer = Instant::now();

        systems::process(&mut world);

        process_requests(&connection);

        while timer.elapsed().as_millis() < constants::LOOP_DURATION_MS.into() {
            sleep(Duration::from_millis(1));
            // TODO get requests, do logic, make responses
        }
    }
}
