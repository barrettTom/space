extern crate space;

use diesel::prelude::*;
use legion::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

use space::constants;
use space::entities;
use space::request::{Request, RequestData};
use space::response::{Response, ResponseData};
use space::schema::requests::dsl;
use space::components::misc::Name;
use space::systems;

fn populate(world: &mut World) {
    for _ in 0..constants::ASTROID_COUNT {
        entities::Astroid::insert_to(world);
    }
}

fn process_requests(world: &mut World, connection: &SqliteConnection) {
    let requests = dsl::requests
        .filter(dsl::received.eq(false))
        .load::<Request>(connection)
        .unwrap();

    if !requests.is_empty() {
        println!("{:?}", requests);
    }

    for request in requests.iter() {
        let request_data = request.get_data();
        let response = match request_data {
            RequestData::Register{user, pass} => {
                match <Read<Name>>::query().iter(world).find(|name| name.0 == user) {
                    Some(_) => Response::new(ResponseData::Bad, request.id().to_string()),
                    None => {
                        entities::Ship::insert_to(user, pass, world);
                        Response::new(ResponseData::Good, request.id().to_string())
                    },
                }

            },
            _ => Response::new(ResponseData::Bad, request.id().to_string()),
        };

        response.insert_into(&connection);
        request.mark_received(&connection);
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

        process_requests(&mut world, &connection);

        while timer.elapsed().as_millis() < constants::LOOP_DURATION_MS.into() {
            sleep(Duration::from_millis(
                constants::LOOP_DURATION_MS - timer.elapsed().as_millis() as u64,
            ));
        }
    }
}
