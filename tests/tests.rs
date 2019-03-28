extern crate diesel;
extern crate migrations_internals;
extern crate space;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::Duration;

    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use space::schema::masses::dsl;
    use space::schema::masses::dsl::masses as db_masses;

    use space::constants;
    use space::item::{Item, ItemType};
    use space::mass::{Mass, MassEntry};
    use space::math::{get_db_url, Vector};
    use space::modules::construction;
    use space::modules::mining;
    use space::modules::navigation;
    use space::modules::refinery;
    use space::modules::tractorbeam;
    use space::modules::types::ModuleType;

    fn setup() -> (Mass, HashMap<String, Mass>) {
        let ship = Mass::new_ship();
        let mut astroid = Mass::new_astroid();
        astroid.position = Vector::default();
        astroid.velocity = Vector::default();

        let mut masses = HashMap::new();
        masses.insert(String::from("astroid"), astroid);
        (ship, masses)
    }

    fn setup_ship_target(ship: &mut Mass, target_name: String, masses: &mut HashMap<String, Mass>) {
        ship.give_received_data(ModuleType::Navigation, target_name);
        ship.process(masses);
        sleep(Duration::from_secs(constants::SHIP_NAVIGATION_TIME + 1));
        ship.process(masses);
    }

    #[test]
    fn test_navigation_range() {
        let (mut ship, mut masses) = setup();

        ship.give_received_data(ModuleType::Navigation, String::from("astroid"));
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Navigation, &masses);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::Targeting);

        let astroid = masses.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Navigation, &masses);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::None);
    }

    #[test]
    fn test_navigation_range_targeted() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        let data = ship.get_client_data(ModuleType::Navigation, &masses);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::Targeted);

        let astroid = masses.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Navigation, &masses);
        let navigation_data: navigation::ClientData = serde_json::from_str(&data).unwrap();
        assert!(navigation_data.status == navigation::Status::None);
    }

    #[test]
    fn test_mining_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Mining, &masses);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::Mining);

        let mut astroid = masses.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_MINING_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Mining, &masses);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::None);
    }

    #[test]
    fn test_mining_navigation_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Mining, &masses);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::Mining);

        let mut astroid = masses.get_mut("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Mining, &masses);
        let mining_data: mining::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == mining::Status::None);
    }

    #[test]
    fn test_mining() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses);
        sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
        ship.process(&mut masses);
        assert!(ship.item_count(ItemType::CrudeMinerals) == 1);

        sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
        ship.process(&mut masses);
        assert!(ship.item_count(ItemType::CrudeMinerals) == 2);
    }

    #[test]
    fn test_mining_storage() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        for _ in 0..10 {
            ship.give_item(Item::new(ItemType::CrudeMinerals));
        }

        ship.give_received_data(ModuleType::Mining, String::from("F"));
        ship.process(&mut masses);
        sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
        assert!(masses.len() == 1);
        ship.process(&mut masses);
        assert!(masses.len() == 2);
    }

    #[test]
    fn test_refinery() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        ship.give_received_data(ModuleType::Refinery, String::from("R"));
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Refinery, &masses);
        let mining_data: refinery::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == refinery::Status::None);

        ship.give_item(Item::new(ItemType::CrudeMinerals));

        ship.give_received_data(ModuleType::Refinery, String::from("R"));
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Refinery, &masses);
        let mining_data: refinery::ClientData = serde_json::from_str(&data).unwrap();
        assert!(mining_data.status == refinery::Status::Refining);

        sleep(Duration::from_secs(constants::SHIP_REFINERY_TIME + 1));
        ship.process(&mut masses);
        assert!(ship.item_count(ItemType::Iron) == 1);
        assert!(ship.item_count(ItemType::Hydrogen) == 1);
    }

    #[test]
    fn test_construction() {
        let (mut ship, mut masses) = setup();

        ship.give_received_data(ModuleType::Construction, String::from("c"));
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Construction, &masses);
        let construction_data: construction::ClientData = serde_json::from_str(&data).unwrap();
        assert!(construction_data.status == construction::Status::None);

        for _ in 0..5 {
            ship.give_item(Item::new(ItemType::Iron));
        }

        ship.give_received_data(ModuleType::Construction, String::from("c"));
        ship.process(&mut masses);

        let data = ship.get_client_data(ModuleType::Construction, &masses);
        let construction_data: construction::ClientData = serde_json::from_str(&data).unwrap();
        assert!(construction_data.status == construction::Status::Constructing);
        assert!(masses.len() == 1);

        sleep(Duration::from_secs(constants::SHIP_CONSTRUCTION_TIME + 1));
        ship.process(&mut masses);
        assert!(masses.len() == 2);
        assert!(ship.item_count(ItemType::Iron) == 0);
    }

    #[test]
    fn test_engines() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.velocity = Vector::new(constants::SHIP_ENGINES_ACCELERATION * 2.0, 0.0, 0.0);
        astroid.process(&mut masses);
        masses.insert(String::from("astroid"), astroid);

        ship.give_received_data(ModuleType::Engines, String::from("c"));
        ship.process(&mut masses);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION);

        ship.process(&mut masses);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * 2.0);

        ship.process(&mut masses);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * 2.0);

        ship.give_received_data(ModuleType::Engines, String::from("s"));
        ship.process(&mut masses);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION);

        ship.process(&mut masses);
        assert!(ship.velocity.x == 0.0);

        ship.process(&mut masses);
        assert!(ship.velocity.x == 0.0);

        ship.give_received_data(ModuleType::Engines, String::from("t"));
        ship.process(&mut masses);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * -1.0);

        ship.process(&mut masses);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * -1.0);

        ship.give_received_data(ModuleType::Engines, String::from("t"));
        ship.process(&mut masses);
        assert!(ship.velocity.x == constants::SHIP_ENGINES_ACCELERATION * -2.0);
    }

    #[test]
    fn test_tractorbeam_push_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("p"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::Push);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_TRACTORBEAM_RANGE + 1.0, 0.0, 0.0);
        astroid.process(&mut masses);
        masses.insert(String::from("astroid"), astroid);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::None);
    }

    #[test]
    fn test_tractorbeam_pull_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("o"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::Pull);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_TRACTORBEAM_RANGE + 1.0, 0.0, 0.0);
        astroid.process(&mut masses);
        masses.insert(String::from("astroid"), astroid);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::None);
    }

    #[test]
    fn test_tractorbeam_bring_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("b"),
            desired_distance: Some(5.0),
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::Bring);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(constants::SHIP_TRACTORBEAM_RANGE + 1.0, 0.0, 0.0);
        astroid.process(&mut masses);
        masses.insert(String::from("astroid"), astroid);
        ship.process(&mut masses);
        let data = ship.get_client_data(ModuleType::Tractorbeam, &masses);
        let tractorbeam_data: tractorbeam::ClientData = serde_json::from_str(&data).unwrap();
        assert!(tractorbeam_data.status == tractorbeam::Status::None);
    }

    #[test]
    fn test_tractorbeam() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        let start = 2.0;
        astroid.velocity = Vector::new(start, 0.0, 0.0);
        astroid.process(&mut masses);
        assert!(astroid.position == Vector::new(start, 0.0, 0.0));
        masses.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("o"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses);

            let mut astroid = masses.remove("astroid").unwrap();
            astroid.process(&mut masses);

            let estimated_velocity = start - (constants::SHIP_TRACTORBEAM_STRENGTH * iterated);
            assert!(
                astroid.velocity.x.abs() - estimated_velocity.abs() < constants::FLOAT_PRECISION
            );
            masses.insert(String::from("astroid"), astroid);

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
            ship.process(&mut masses);

            let mut astroid = masses.remove("astroid").unwrap();
            astroid.process(&mut masses);

            let estimated_velocity = start + (constants::SHIP_TRACTORBEAM_STRENGTH * iterated);
            assert!(
                astroid.velocity.x.abs() - estimated_velocity.abs() < constants::FLOAT_PRECISION
            );
            masses.insert(String::from("astroid"), astroid);

            iterated += 1.0;
            if iterated > 10.0 {
                break;
            }
        }
    }

    #[test]
    fn test_tractorbeam_bring() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, String::from("astroid"), &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        let start = 25.0;
        let desired_distance = 5.0;
        astroid.position = Vector::new(start, 0.0, 0.0);
        astroid.process(&mut masses);
        masses.insert(String::from("astroid"), astroid);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("b"),
            desired_distance: Some(desired_distance),
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);
        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses);

            let mut astroid = masses.remove("astroid").unwrap();
            astroid.process(&mut masses);

            if ship.position.distance_from(astroid.position.clone()) < desired_distance
                && astroid.velocity.magnitude() < 1.0
            {
                break;
            }

            masses.insert(String::from("astroid"), astroid);

            iterated += 1.0;
            if iterated > 100.0 {
                assert!(false);
                break;
            }
        }

        /* for plotting
        //let mut xy = Vec::new();
        //xy.push((iterated, astroid.position.x));
        let l =
            plotlib::line::Line::new(&xy[..]).style(plotlib::style::LineStyle::new().colour("red"));
        let v = plotlib::view::ContinuousView::new().add(&l);
        plotlib::page::Page::single(&v)
            .save("line.svg")
            .expect("error");
        std::process::Command::new("feh").arg("--conversion-timeout").arg("1").arg("line.svg").output().expect("problem");
        */
    }

    #[test]
    fn test_tractorbeam_acquire() {
        let (mut ship, mut masses) = setup();
        masses.insert(
            String::from("iron"),
            Mass::new_item(
                Item::new(ItemType::Iron),
                Vector::default(),
                Vector::default(),
            ),
        );
        setup_ship_target(&mut ship, String::from("iron"), &mut masses);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("a"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);

        assert!(masses.len() == 2);
        assert!(ship.item_count(ItemType::Iron) == 0);
        ship.process(&mut masses);
        assert!(ship.item_count(ItemType::Iron) == 1);
        assert!(masses.len() == 1);
    }

    #[test]
    fn test_tractorbeam_acquire_range() {
        let (mut ship, mut masses) = setup();
        masses.insert(
            String::from("iron"),
            Mass::new_item(
                Item::new(ItemType::Iron),
                Vector::new(50.0, 0.0, 0.0),
                Vector::default(),
            ),
        );
        setup_ship_target(&mut ship, String::from("iron"), &mut masses);

        let recv = serde_json::to_string(&tractorbeam::ServerRecvData {
            key: String::from("a"),
            desired_distance: None,
        })
        .unwrap();
        ship.give_received_data(ModuleType::Tractorbeam, recv);

        assert!(masses.len() == 2);
        assert!(ship.item_count(ItemType::Iron) == 0);
        ship.process(&mut masses);
        assert!(ship.item_count(ItemType::Iron) == 0);
        assert!(masses.len() == 2);

        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses);

            match masses.remove("iron") {
                Some(mut item) => {
                    item.process(&mut masses);
                    masses.insert(String::from("iron"), item);
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
        let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");

        let masses = db_masses
            .load::<MassEntry>(&connection)
            .expect("Cannot query, probably no migrations, run 'cargo run --bin migrate'");
        let size = masses.len();
        let name = String::from("test");

        diesel::insert_into(db_masses)
            .values(&Mass::new_astroid().to_new_mass_entry(name.clone()))
            .execute(&connection)
            .expect("Cannot insert");

        let len = db_masses
            .load::<MassEntry>(&connection)
            .expect("Cannot query")
            .len();

        assert!(len == size + 1);

        diesel::delete(db_masses.filter(dsl::name.eq(name)))
            .execute(&connection)
            .expect("Cannot delete");

        let len = db_masses
            .load::<MassEntry>(&connection)
            .expect("Cannot query")
            .len();

        assert!(len == size);
    }
}
