extern crate space;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::Duration;

    use space::constants;
    use space::item::{Item, ItemType};
    use space::mass::Mass;
    use space::math::Vector;
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

    fn setup_ship_target(ship: &mut Mass, masses: &mut HashMap<String, Mass>) {
        ship.give_received_data(ModuleType::Navigation, String::from("astroid"));
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
        setup_ship_target(&mut ship, &mut masses);

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
        setup_ship_target(&mut ship, &mut masses);

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
        setup_ship_target(&mut ship, &mut masses);

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
        setup_ship_target(&mut ship, &mut masses);

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
        setup_ship_target(&mut ship, &mut masses);

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
        setup_ship_target(&mut ship, &mut masses);

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
        setup_ship_target(&mut ship, &mut masses);

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
        setup_ship_target(&mut ship, &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.insert(String::from("astroid"), astroid);

        ship.give_received_data(ModuleType::Tractorbeam, String::from("p"));
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
        setup_ship_target(&mut ship, &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.insert(String::from("astroid"), astroid);

        ship.give_received_data(ModuleType::Tractorbeam, String::from("o"));
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
        setup_ship_target(&mut ship, &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        astroid.position = Vector::new(1.0, 0.0, 0.0);
        masses.insert(String::from("astroid"), astroid);

        ship.give_received_data(ModuleType::Tractorbeam, String::from("b"));
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
        setup_ship_target(&mut ship, &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        let start = 2.0;
        astroid.velocity = Vector::new(start, 0.0, 0.0);
        astroid.process(&mut masses);
        assert!(astroid.position == Vector::new(start, 0.0, 0.0));
        masses.insert(String::from("astroid"), astroid);

        ship.give_received_data(ModuleType::Tractorbeam, String::from("o"));
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

        ship.give_received_data(ModuleType::Tractorbeam, String::from("p"));
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
        setup_ship_target(&mut ship, &mut masses);

        let mut astroid = masses.remove("astroid").unwrap();
        let start = 25.0;
        astroid.position = Vector::new(start, 0.0, 0.0);
        astroid.process(&mut masses);
        masses.insert(String::from("astroid"), astroid);

        ship.give_received_data(ModuleType::Tractorbeam, String::from("b"));
        let mut iterated = 1.0;
        loop {
            ship.process(&mut masses);

            let mut astroid = masses.remove("astroid").unwrap();
            astroid.process(&mut masses);

            if ship.position.distance_from(astroid.position.clone())
                < constants::SHIP_TRACTORBEAM_BRING_TO_DISTANCE
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
}