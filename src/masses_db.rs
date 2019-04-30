use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};

use std::io::Write;
use std::thread::spawn;
use std::time::SystemTime;

use crate::constants;
use crate::db::{get_db_url, MassEntry};
use crate::mass::Mass;
use crate::math::{rand_name, Vector};
use crate::server_connection::ServerConnection;

use crate::schema::masses::dsl as masses_dsl;
use crate::schema::masses::dsl::masses as masses_db;
use crate::schema::masses::dsl::name as masses_name;

pub enum Init {
    None,
    Test,
    Restore,
    Populate,
}

pub struct Masses {
    pub hashmap: HashMap<String, Mass>,
    connection: PgConnection,
}

impl Masses {
    pub fn new(init: Init) -> Masses {
        let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");

        let hashmap = match init {
            Init::None => HashMap::new(),
            Init::Restore => masses_db
                .load::<MassEntry>(&connection)
                .expect("Cannot query, are you sure you can restore?")
                .iter()
                .map(MassEntry::to_mass)
                .collect(),
            Init::Populate => {
                let mut hashmap: HashMap<String, Mass> = HashMap::new();
                for _ in 0..constants::ASTROID_COUNT {
                    hashmap.insert(rand_name(), Mass::new_astroid());
                }
                hashmap
            }
            Init::Test => {
                let mut hashmap: HashMap<String, Mass> = HashMap::new();
                let mut astroid = Mass::new_astroid();
                astroid.position = Vector::default();
                astroid.velocity = Vector::default();
                hashmap.insert(String::from("astroid"), astroid);
                hashmap
            },
        };

        Masses {
            hashmap,
            connection,
        }
    }

    pub fn process(&mut self) {
        for key in self.hashmap.clone().keys() {
            let mut mass = self.hashmap.remove(key).unwrap();
            mass.process(&mut self.hashmap);
            self.hashmap.insert(key.to_string(), mass);
        }
    }

    pub fn communicate(&mut self, connection: &mut ServerConnection) {
        let mut ship = self.hashmap.remove(&connection.name).unwrap();
        let send = ship.get_client_data(connection.module_type.clone(), &self.hashmap);

        connection.open = connection.stream.write(send.as_bytes()).is_ok();

        let recv = connection.receive();
        ship.give_received_data(connection.module_type.clone(), recv);

        self.hashmap.insert(connection.name.clone(), ship);
    }

    pub fn backup(&self) {
        let hashmap_clone = self.hashmap.clone();
        let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");
        spawn(move || {
            let timestamp = SystemTime::now();
            for (name, mass) in hashmap_clone {
                let mass_entry = mass.to_mass_entry(name.to_string(), timestamp);
                diesel::insert_into(masses_db)
                    .values(&mass_entry)
                    .on_conflict(masses_name)
                    .do_update()
                    .set(&mass_entry)
                    .execute(&connection)
                    .expect("Cannot backup");
            }
        });
    }

    pub fn len(&self) -> usize {
        self.all().len()
    }

    pub fn is_empty(&self) -> bool {
        self.all().len() == 0
    }

    pub fn all(&self) -> Vec<MassEntry> {
        all(&self.connection)
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
}

pub fn all(connection: &PgConnection) -> Vec<MassEntry> {
    masses_db
        .load::<MassEntry>(connection)
        .expect("Cannot query.")
}
