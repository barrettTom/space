extern crate space;

#[cfg(test)]
mod test {
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
        astroid.position = Vector::new((constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0));
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
        astroid.position = Vector::new((constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0));
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
        astroid.position = Vector::new((constants::SHIP_MINING_RANGE + 1.0, 0.0, 0.0));
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
}
