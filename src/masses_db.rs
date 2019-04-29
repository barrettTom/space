use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use std::time::SystemTime;

use crate::constants;
use crate::db::{MassEntry, get_db_url};
use crate::mass::Mass;
use crate::math::rand_name;

use crate::schema::masses::dsl as masses_dsl;
use crate::schema::masses::dsl::masses as masses_db;
use crate::schema::masses::dsl::name as masses_name;

pub struct MassesDB {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl MassesDB {
    pub fn new(connection: Option<PooledConnection<ConnectionManager<PgConnection>>>) -> MassesDB {
        let connection = match connection {
            Some(connection) => connection,
            None => Pool::new(ConnectionManager::<PgConnection>::new(get_db_url()))
                .unwrap()
                .get()
                .unwrap(),
        };

        MassesDB { connection }
    }

    pub fn len(&self) -> usize {
        self.all().len()
    }

    pub fn is_empty(&self) -> bool {
        self.all().len() == 0
    }

    pub fn all(&self) -> Vec<MassEntry> {
        masses_db
            .load::<MassEntry>(&self.connection)
            .expect("Cannot query.")
    }

    pub fn get(&self, name: String) -> MassEntry {
        masses_db
            .filter(masses_dsl::name.eq(name))
            .load::<MassEntry>(&self.connection)
            .expect("Cannot filter")
            .pop()
            .unwrap()
    }

    pub fn insert(&self, mass_entry: MassEntry) {
        mass_entry.insert_into(&self.connection)
    }

    pub fn update(&self, mass_entry: MassEntry) {
        mass_entry.update(&self.connection)
    }

    pub fn delete(&self, mass_entry: MassEntry) {
        mass_entry.delete(&self.connection)
    }

    pub fn backup(&self, masses: HashMap<String, Mass>) {
        let timestamp = SystemTime::now();
        for (name, mass) in masses {
            let mass_entry = mass.to_mass_entry(name.to_string(), timestamp);
            diesel::insert_into(masses_db)
                .values(&mass_entry)
                .on_conflict(masses_name)
                .do_update()
                .set(&mass_entry)
                .execute(&self.connection)
                .expect("Cannot backup");
        }
    }

    pub fn restore(&self) -> HashMap<String, Mass> {
        masses_db
            .load::<MassEntry>(&self.connection)
            .expect("Cannot query, are you sure you can restore?")
            .iter()
            .map(MassEntry::to_mass)
            .collect()
    }

    pub fn populate(&self) -> HashMap<String, Mass> {
        let mut masses: HashMap<String, Mass> = HashMap::new();

        for _ in 0..constants::ASTROID_COUNT {
            masses.insert(rand_name(), Mass::new_astroid());
        }

        masses
    }
}

impl Default for MassesDB {
    fn default() -> Self {
        Self::new(None)
    }
}
