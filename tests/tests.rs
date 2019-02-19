extern crate space;

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::Duration;

    use space::constants;
    use space::item::{Item, ItemType};
    use space::mass::{Mass, MassType};
    use space::math::Vector;
    use space::modules::mining::MiningStatus;
    use space::modules::refinery::RefineryStatus;
    use space::modules::navigation::NavigationStatus;
    use space::modules::construction::ConstructionStatus;

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
        if let MassType::Ship {
            ref mut navigation, ..
        } = ship.mass_type
        {
            let astroid = masses.get_mut("astroid").unwrap();
            astroid.position = Vector::default();
            navigation.give_received_data(String::from("astroid"));
            navigation.process(ship.position.clone(), masses);
            sleep(Duration::from_secs(constants::SHIP_NAVIGATION_TIME + 1));
            navigation.process(ship.position.clone(), masses);
        }
    }

    #[test]
    fn test_navigation_range() {
        let (mut ship, mut masses) = setup();

        if let MassType::Ship {
            ref mut navigation, ..
        } = ship.mass_type
        {
            navigation.give_received_data(String::from("astroid"));
            navigation.process(ship.position.clone(), &mut masses);
            assert!(navigation.status == NavigationStatus::Targeting);

            let astroid = masses.get_mut("astroid").unwrap();
            astroid.position = Vector::new((constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0));
            navigation.process(ship.position.clone(), &mut masses);
            assert!(navigation.status == NavigationStatus::None);
        }
    }

    #[test]
    fn test_navigation_range_targeted() {
        let (mut ship, mut masses) = setup();

        setup_ship_target(&mut ship, &mut masses);
        if let MassType::Ship {
            ref mut navigation, ..
        } = ship.mass_type
        {
            assert!(navigation.status == NavigationStatus::Targeted);

            let astroid = masses.get_mut("astroid").unwrap();
            astroid.position = Vector::new((constants::SHIP_NAVIGATION_RANGE + 1.0, 0.0, 0.0));
            navigation.process(ship.position.clone(), &mut masses);
            assert!(navigation.status == NavigationStatus::None);
        }
    }

    #[test]
    fn test_mining_range() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, &mut masses);

        if let MassType::Ship {
            ref mut storage,
            ref mut mining,
            ..
        } = ship.mass_type
        {
            mining.give_received_data(String::from("F"));
            assert!(mining.status == MiningStatus::Mining);

            let mut astroid = masses.remove("astroid").unwrap();
            astroid.position = Vector::new((constants::SHIP_MINING_RANGE + 1.0, 0.0, 0.0));
            mining.process(ship.position.clone(), &mut masses, &mut astroid, storage);
            assert!(mining.status == MiningStatus::None);
        }
    }

    #[test]
    fn test_mining() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, &mut masses);

        if let MassType::Ship {
            ref mut storage,
            ref mut mining,
            ..
        } = ship.mass_type
        {
            mining.give_received_data(String::from("F"));
            assert!(mining.status == MiningStatus::Mining);

            let mut astroid = masses.remove("astroid").unwrap();
            sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
            mining.process(ship.position.clone(), &mut masses, &mut astroid, storage);
            assert!(
                storage
                    .items
                    .iter()
                    .filter(|item| item.item_type == ItemType::CrudeMinerals)
                    .count()
                    == 1
            );

            sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
            mining.process(ship.position.clone(), &mut masses, &mut astroid, storage);
            assert!(
                storage
                    .items
                    .iter()
                    .filter(|item| item.item_type == ItemType::CrudeMinerals)
                    .count()
                    == 2
            );
        }
    }

    #[test]
    fn test_mining_storage() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, &mut masses);

        if let MassType::Ship {
            ref mut storage,
            ref mut mining,
            ..
        } = ship.mass_type
        {
            for _ in 0..10 {
                storage.give_item(Item::new(ItemType::CrudeMinerals));
            }

            mining.give_received_data(String::from("F"));

            let mut astroid = masses.remove("astroid").unwrap();
            sleep(Duration::from_secs(constants::SHIP_MINING_TIME + 1));
            assert!(masses.len() == 0);
            mining.process(ship.position.clone(), &mut masses, &mut astroid, storage);
            assert!(masses.len() == 1);
        }
    }

    #[test]
    fn test_refinery() {
        let (mut ship, mut masses) = setup();
        setup_ship_target(&mut ship, &mut masses);

        if let MassType::Ship {
            ref mut storage,
            ref mut refinery,
            ..
        } = ship.mass_type
        {
            refinery.give_received_data(String::from("R"));
            refinery.process(storage);
            assert!(refinery.status == RefineryStatus::None);

            storage.give_item(Item::new(ItemType::CrudeMinerals));

            refinery.give_received_data(String::from("R"));
            refinery.process(storage);
            assert!(refinery.status == RefineryStatus::Refining);

            sleep(Duration::from_secs(constants::SHIP_REFINERY_TIME + 1));
            refinery.process(storage);
            assert!(storage
                .items
                .iter()
                .any(|item| item.item_type == ItemType::Iron));
            assert!(storage
                .items
                .iter()
                .any(|item| item.item_type == ItemType::Hydrogen));
        }
    }

    #[test]
    fn test_construction() {
        let (mut ship, mut masses) = setup();

        if let MassType::Ship {
            ref mut storage,
            ref mut construction,
            ..
        } = ship.mass_type
        {
            construction.give_received_data(String::from("c"));
            construction.process(ship.velocity.clone(), ship.position.clone(), &mut masses, storage);
            assert!(construction.status == ConstructionStatus::None);

            for _ in 0..5 {
                storage.give_item(Item::new(ItemType::Iron));
            }

            construction.give_received_data(String::from("c"));
            construction.process(ship.velocity.clone(), ship.position.clone(), &mut masses, storage);
            assert!(construction.status == ConstructionStatus::Constructing);
            assert!(masses.len() == 1);


            sleep(Duration::from_secs(constants::SHIP_CONSTRUCTION_TIME + 1));
            construction.process(ship.velocity.clone(), ship.position.clone(), &mut masses, storage);
            assert!(masses.len() == 2);
        }
    }
}
