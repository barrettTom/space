extern crate space;

use diesel::prelude::*;
use legion::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

use space::components::misc::{Name, Pass};
use space::constants;
use space::entities;
use space::request::{Request, RequestData};
use space::response::{Response, ResponseData};
use space::schema::requests::dsl;
use space::systems;

fn populate(world: &mut World) {
    for _ in 0..constants::ASTROID_COUNT {
        entities::Astroid::insert_to(world);
    }
}

fn process_requests(world: &mut World, connection: &SqliteConnection) {
    if let Ok(requests) = dsl::requests
        .filter(dsl::received.eq(false))
        .load::<Request>(connection)
    {
        if !requests.is_empty() {
            println!("{:?}", requests);
        }

        for request in requests.iter() {
            let request_data = request.get_data();
            let response = match request_data {
                RequestData::Register { user, pass } => {
                    let exists = <Read<Name>>::query().iter(world).any(|name| name.0 == user);
                    if exists {
                        let response_data = ResponseData::new("Conflict", "Already Exists", None);
                        Response::new(response_data, request.id().to_string())
                    } else {
                        entities::Ship::insert_to(user, pass, world);
                        let response_data = ResponseData::new("Ok", "", None);
                        Response::new(response_data, request.id().to_string())
                    }
                }
                RequestData::Play { user, pass, module } => {
                    let authd = <(Read<Name>, Read<Pass>)>::query()
                        .iter(world)
                        .any(|(name, passs)| name.0 == user && passs.0 == pass);
                    if authd {
                        let response_data = match module.as_str() {
                            "dashboard" => ResponseData::new("Ok", "", None),
                            _ => ResponseData::new("Not Implemented", "", None),
                        };
                        Response::new(response_data, request.id().to_string())
                    } else {
                        let response_data =
                            ResponseData::new("Unauthorized", "Username/Password is wrong.", None);
                        Response::new(response_data, request.id().to_string())
                    }
                }
            };

            response.insert_into(&connection);
            request.mark_received(&connection);
        }
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
