extern crate diesel;
extern crate migrations_internals;
extern crate space;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    use diesel::pg::PgConnection;
    use diesel::r2d2::{ConnectionManager, Pool};

    use space::constants;
    use space::db::{encrypt, get_db_url, verify, Login, Registration};
    use space::item::{Item, ItemType};
    use space::mass::Mass;
    use space::masses_db::{Init, Masses};
    use space::math::Vector;
    use space::modules::construction;
    use space::modules::mining;
    use space::modules::navigation;
    use space::modules::refinery;
    use space::modules::tractorbeam;
    use space::modules::types::ModuleType;
    use space::server_connection::ServerConnection;

    fn setup() -> (Mass, Masses) {
        (Mass::new_ship(), Masses::new(Init::Test))
    }

    fn setup_ship_target(
        ship: &mut Mass,
        target_name: String,
        hashmap: &mut HashMap<String, Mass>,
    ) {
        ship.give_received_data(ModuleType::Navigation, target_name);
        ship.process(hashmap);
        sleep(Duration::from_secs(constants::SHIP_NAVIGATION_TIME + 1));
        ship.process(hashmap);
    }

    #[test]
    fn test_navigation_range() {
        let (mut ship, mut masses) = setup();

        ship.give_received_data(ModuleType::Navigation, String::from("astroid"));
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Navigation, &masses.hashmap);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::Targeting);

        let astroid = masses.hashmap.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Navigation, &masses.hashmap);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::None);
    }

    #[test]
    fn test_navigation_range_targeted() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        let data = ship.get_client_data(ModuleType::Navigation, &masses.hashmap);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::Targeted);

        let astroid = masses.hashmap.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Navigation, &masses.hashmap);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::None);
    }

    #[test]
    fn test_mining_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Mining, &masses.hashmap);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::Mining);

        let mut astroid = masses.hashmap.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_MINING_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Mining, &masses.hashmap);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::None);
    }

    #[test]
    fn test_mining_navigation_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Mining, &masses.hashmap);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::Mining);

        let mut astroid = masses.hashmap.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Mining, &masses.hashmap);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::None);
    }

    #[test]
    fn test_mining() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses.hashmap);
        sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
        ship.process(&mut masses.hashmap);
        assert!(ship.item_count(ItemType::CrudeMinerals) == 1);

        sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
        ship.process(&mut masses.hashmap);
        assert!(ship.item_count(ItemType::CrudeMinerals) == 2);
    }

    #[test]
    fn test_mining_storage() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        for _ in 0..10 {
            ship.give_item(Item::new(ItemType::CrudeMinerals));
        }

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses.hashmap);
        sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
        assert!(masses.hashmap.len() == 1);
        ship.process(&mut masses.hashmap);
        assert!(masses.hashmap.len() == 2);
    }

    #[test]
    fn test_refinery() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        ship.give_received_data(ModuleType::Refinery, String::from("R"));
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Refinery, &masses.hashmap);
        let mining_data: refinery::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == refinery::Status::None);

        ship.give_item(Item::new(ItemType::CrudeMinerals));

        ship.give_received_data(ModuleType::Refinery, String::from("R"));
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Refinery, &masses.hashmap);
        let mining_data: refinery::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == refinery::Status::Refining);

        sleep(Duration::from_secs(constants::SHIP_REFINERY_TIME + 1));
        ship.process(&mut masses.hashmap);
        assert!(ship.item_count(ItemType::Iron) == 1);
        assert!(ship.item_count(ItemType::Hydrogen) == 1);
    }

    #[test]
    fn test_construction() {
        let (mut ship, mut masses) = setup();

        ship.give_received_data(ModuleType::Construction, String::from("c"));
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Construction, &masses.hashmap);
        let construction_data: construction::ClientData = serde_json::from_str(&data).unwrap();
        assert!(construction_data.status == construction::Status::None);

        for _ in 0..5 {
            ship.give_item(Item::new(ItemType::Iron));
        }

        ship.give_received_data(ModuleType::Construction, String::from("c"));
        ship.process(&mut masses.hashmap);

        let data = ship.get_client_data(ModuleType::Construction, &masses.hashmap);
        let construction_data: construction::ClientData = serde_json::from_str(&data).unwrap();
        assert!(construction_data.status == construction::Status::Constructing);
        assert!(masses.hashmap.len() == 1);

        sleep(Duration::from_secs(constants::SHIP_CONSTRUCTION_TIME + 1));
        ship.process(&mut masses.hashmap);
        assert!(masses.hashmap.len() == 2);
        assert!(ship.item_count(ItemType::Iron) == 0);
    }

    #[test]
    fn test_engines() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        astroid.velocity = Vector::new(constants::SHIP_ENGINES_ACCELERATION * 2.0, 0.0, 0.0);
        astroid.process(&mut masses.hashmap);
        masses.hashmap.insert(String::from("astroid"), astroid);

        ship.give_received_data(ModuleType::Engines, String::from("c"));
        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION);

        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * 2.0);

        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * 2.0);

        ship.give_received_data(ModuleType::Engines, String::from("s"));
        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION);

        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == 0.0);

        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == 0.0);

        ship.give_received_data(ModuleType::Engines, String::from("t"));
        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * -1.0);

        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * -1.0);

        ship.give_received_data(ModuleType::Engines, String::from("t"));
        ship.process(&mut masses.hashmap);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * -2.0);
    }

    #[test]
    fn test_tractorbeam_push_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.hashmap.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("p"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses.hashmap);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::Push);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_TRACTORBEAM_RANGE + 1.0, 0.0, 0.0);
        astroid.process(&mut masses.hashmap);
        masses.hashmap.insert(String::from("astroid"), astroid);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses.hashmap);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::None);
    }

    #[test]
    fn test_tractorbeam_pull_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.hashmap.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("o"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses.hashmap);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::Pull);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_TRACTORBEAM_RANGE + 1.0, 0.0, 0.0);
        astroid.process(&mut masses.hashmap);
        masses.hashmap.insert(String::from("astroid"), astroid);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses.hashmap);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::None);
    }

    #[test]
    fn test_tractorbeam_bring_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.hashmap.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("b"),
            desired_distance: Some(5.0),
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses.hashmap);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::Bring);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_TRACTORBEAM_RANGE + 1.0, 0.0, 0.0);
        astroid.process(&mut masses.hashmap);
        masses.hashmap.insert(String::from("astroid"), astroid);
        ship.process(&mut masses.hashmap);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses.hashmap);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::None);
    }

    #[test]
    fn test_tractorbeam() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        let start = 2.0;
        astroid.velocity = Vector::new(start, 0.0, 0.0);
        astroid.process(&mut masses.hashmap);
        assert!(astroid.position == Vector::new(start, 0.0, 0.0));
        masses.hashmap.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("o"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses.hashmap);

            let mut astroid = masses.hashmap.remove("astroid").unwrap();
            astroid.process(&mut masses.hashmap);

            let estimated_velocity = start - (constants::SHIP_TRACTORBEAM_STRENGTH * iterated);
            assert!(
                astroid.velocity.x.abs() - estimated_velocity.abs() < constants::FLOAT_PRECISION
            );
            masses.hashmap.insert(String::from("astroid"), astroid);

            iterated += 1.0;
            if iterated > 10.0 {
                break;
            }
        }

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("p"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses.hashmap);

            let mut astroid = masses.hashmap.remove("astroid").unwrap();
            astroid.process(&mut masses.hashmap);

            let estimated_velocity = start + (constants::SHIP_TRACTORBEAM_STRENGTH * iterated);
            assert!(
                astroid.velocity.x.abs() - estimated_velocity.abs() < constants::FLOAT_PRECISION
            );
            masses.hashmap.insert(String::from("astroid"), astroid);

            iterated += 1.0;
            if iterated > 10.0 {
                break;
            }
        }
    }

    #[test]
    fn test_tractorbeam_bring() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses.hashmap);

        let mut astroid = masses.hashmap.remove("astroid").unwrap();
        let start = 25.0;
        let desired_distance = 5.0;
        astroid.position = Vector::new(start, 0.0, 0.0);
        astroid.process(&mut masses.hashmap);
        masses.hashmap.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("b"),
            desired_distance: Some(desired_distance),
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses.hashmap);

            let mut astroid = masses.hashmap.remove("astroid").unwrap();
            astroid.process(&mut masses.hashmap);

            if ship.position.distance_from(astroid.position.clone()) < desired_distance
                && astroid.velocity.magnitude() < 1.0
            {
                break;
            }

            masses.hashmap.insert(String::from("astroid"), astroid);

            iterated += 1.0;
            if iterated > 100.0 {
                assert!(false);
                break;
            }
        }
    }

    #[test]
    fn test_tractorbeam_acquire() {
        let (mut ship, mut masses) = setup();
        masses.hashmap.insert(
            String::from("iron"),
            Mass::new_item(
                Item::new(ItemType::Iron),
                Vector::default(),
                Vector::default(),
            ),
        );
        setup_ship_target(&mut ship, String::from("iron"), &mut masses.hashmap);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("a"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);

        assert!(masses.hashmap.len() == 2);
        assert!(ship.item_count(ItemType::Iron) == 0);
        ship.process(&mut masses.hashmap);
        assert!(ship.item_count(ItemType::Iron) == 1);
        assert!(masses.hashmap.len() == 1);
    }

    #[test]
    fn test_tractorbeam_acquire_range() {
        let (mut ship, mut masses) = setup();
        masses.hashmap.insert(
            String::from("iron"),
            Mass::new_item(
                Item::new(ItemType::Iron),
                Vector::new(50.0, 0.0, 0.0),
                Vector::default(),
            ),
        );
        setup_ship_target(&mut ship, String::from("iron"), &mut masses.hashmap);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("a"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);

        assert!(masses.hashmap.len() == 2);
        assert!(ship.item_count(ItemType::Iron) == 0);
        ship.process(&mut masses.hashmap);
        assert!(ship.item_count(ItemType::Iron) == 0);
        assert!(masses.hashmap.len() == 2);

        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses.hashmap);

            match masses.hashmap.remove("iron") {
                Some(mut item) => {
                    item.process(&mut masses.hashmap);
                    masses.hashmap.insert(String::from("iron"), item);
                }
                None => {
                    assert!(ship.item_count(ItemType::Iron) == 1);
                    break;
                }
            }

            iterated += 1.0;
            if iterated > 100.0 {
                assert!(false);
                break;
            }
        }
    }

    #[test]
    fn test_postgres() {
        let masses = Masses::new(Init::None);
        let len_before = masses.hashmap.len();

        let name = String::from("test");
        let mut mass = Mass::new_astroid();

        masses.insert(mass.to_mass_entry(name.clone(), SystemTime::now()));

        assert!(masses.len() == len_before + 1);

        let db_mass = masses.get(name.clone());

        assert!(mass.position.x == db_mass.to_mass().1.position.x);

        mass.process(&mut HashMap::new());

        masses.update(mass.to_mass_entry(name.clone(), SystemTime::now()));

        let db_mass = masses.get(name.clone());

        assert!(mass.position.x == db_mass.to_mass().1.position.x);

        masses.delete(db_mass);

        assert!(masses.hashmap.len() == len_before);
    }

    #[test]
    fn test_salt_hash() {
        let mut password = String::from("this is a test password");
        let (hash, salt) = encrypt(password.clone());
        assert!(verify(password.clone(), hash.clone(), salt.clone()).is_ok());
        password.push_str("bad");
        assert!(!verify(password, hash, salt).is_ok());
    }

    #[test]
    fn test_register_login_user() {
        let pool = Pool::new(ConnectionManager::<PgConnection>::new(get_db_url())).unwrap();
        let name = String::from("test");
        let pass = String::from("test");
        let user = Registration {
            name: name.clone(),
            email: "test@gmail.com".to_string(),
            password1: pass.clone(),
            password2: pass.clone(),
        }.to_user().expect("Cannot create");

        user.insert_into(pool.get().unwrap()).expect("Cannot insert");

        let mut form = Login {
            name: name.clone(),
            password: pass.clone(),
        };
        assert!(form.verify(pool.get().unwrap()).is_ok());

        form.password.push_str("zz");
        assert!(!form.verify(pool.get().unwrap()).is_ok());

        form.name.push_str("zz");
        assert!(!form.verify(pool.get().unwrap()).is_ok());

        user.delete(pool.get().unwrap());
    }

    #[test]
    fn test_server_connection() {
        let (_, mut masses) = setup();

        let listener = TcpListener::bind(constants::SERVER_IP_PORT).unwrap();
        let mut client_stream =
            TcpStream::connect(constants::SERVER_IP_PORT).expect("Cannot connect");

        let name = String::from("tom");
        let pass = String::from("pass");

        let send = name.clone()
            + ":"
            + &pass
            + ":"
            + &serde_json::to_string(&ModuleType::Mining).unwrap()
            + "\n";

        client_stream.write_all(send.as_bytes()).unwrap();
        if let Ok((stream, _)) = listener.accept() {
            assert!(ServerConnection::new(stream, &mut masses).is_none());
        }

        let pool = Pool::new(ConnectionManager::<PgConnection>::new(get_db_url())).unwrap();

        let user = Registration {
            name: name.clone(),
            email: "spalf0@gmail.com".to_string(),
            password1: pass.clone(),
            password2: pass.clone(),
        }.to_user().expect("Cannot create");

        user.insert_into(pool.get().unwrap()).expect("Cannot insert");

        //client_stream.write_all(send.as_bytes()).unwrap();
        //if let Ok((stream, _)) = listener.accept() {
        //    assert!(ServerConnection::new(stream, &mut masses).is_none());
        //}

        user.delete(pool.get().unwrap());
    }
}
