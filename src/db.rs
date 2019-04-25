use data_encoding::HEXUPPER;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand as ring_rand};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::time::SystemTime;

use crate::constants;
use crate::mass::Mass;
use crate::math::rand_name;

use crate::schema::masses as masses_schema;
use crate::schema::masses::dsl as masses_dsl;
use crate::schema::masses::dsl::masses as masses_db;
use crate::schema::masses::dsl::name as masses_name;

use crate::schema::users as users_schema;
use crate::schema::users::dsl as users_dsl;
use crate::schema::users::dsl::users as users_db;

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, Debug)]
#[belongs_to(User)]
#[table_name = "masses_schema"]
pub struct MassEntry {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub name: String,
    pub mass: String,
    pub last_modified: SystemTime,
}

impl MassEntry {
    pub fn to_mass(&self) -> (String, Mass) {
        (self.name.clone(), serde_json::from_str(&self.mass).unwrap())
    }

    pub fn insert_into(&self, connection: &PgConnection) {
        diesel::insert_into(masses_db)
            .values(self)
            .execute(connection)
            .expect("Cannot insert");
    }

    pub fn update(&self, connection: &PgConnection) {
        diesel::update(masses_db)
            .set(self)
            .execute(connection)
            .expect("Cannot update");
    }

    pub fn delete(&self, connection: &PgConnection) {
        diesel::delete(masses_db.filter(masses_dsl::name.eq(self.name.clone())))
            .execute(connection)
            .expect("Cannot delete.");
    }
}

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

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Debug)]
#[table_name = "users_schema"]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub hash: String,
    pub salt: String,
    pub email: String,
    pub created: SystemTime,
}

impl User {
    pub fn insert_into(
        &self,
        connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), String> {
        match diesel::insert_into(users_db)
            .values(self)
            .execute(&connection)
        {
            Ok(_) => Ok(()),
            Err(_error) => Err(String::from("Already exists.")),
        }
    }

    pub fn delete(&self, connection: PooledConnection<ConnectionManager<PgConnection>>) {
        diesel::delete(users_db.filter(users_dsl::name.eq(self.name.clone())))
            .execute(&connection)
            .expect("Cannot delete.");
    }
}

#[derive(Deserialize)]
pub struct Login {
    pub name: String,
    pub password: String,
}

impl Login {
    pub fn verify(
        &self,
        connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), String> {
        match users_db
            .filter(users_dsl::name.eq(self.name.clone()))
            .load::<User>(&connection)
        {
            Ok(user) => {
                if let false = user.is_empty() {
                    verify(
                        self.password.clone(),
                        user[0].hash.clone(),
                        user[0].salt.clone(),
                    )
                } else {
                    Err(String::from("Username not found."))
                }
            }
            Err(_) => Err(String::from("Username not found.")),
        }
    }
}

#[derive(Deserialize)]
pub struct Registration {
    pub name: String,
    pub email: String,
    pub password1: String,
    pub password2: String,
}

impl Registration {
    pub fn to_user(&self) -> Result<User, String> {
        if self.password1 == self.password2 {
            let (hash, salt) = encrypt(self.password1.clone());
            Ok(User {
                id: None,
                name: self.name.clone(),
                email: self.email.clone(),
                hash,
                salt,
                created: SystemTime::now(),
            })
        } else {
            Err(String::from("Passwords not equal"))
        }
    }

    pub fn to_user_and_insert_into(
        &self,
        connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), String> {
        let user = self.to_user()?;
        user.insert_into(connection)
    }
}

pub fn get_db_url() -> String {
    let mut db_url = String::new();
    db_url.push_str("postgres://");
    db_url.push_str(constants::POSTGRES_USERNAME);
    db_url.push_str(":");
    db_url.push_str(constants::POSTGRES_PASSWORD);
    db_url.push_str("@");
    db_url.push_str(constants::POSTGRES_IP);
    db_url.push_str("/");
    db_url.push_str(constants::POSTGRES_DB_NAME);
    db_url
}

pub fn verify(password: String, hash: String, salt: String) -> Result<(), String> {
    match pbkdf2::verify(
        &digest::SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &HEXUPPER.decode(&salt.as_bytes()).unwrap(),
        password.as_bytes(),
        &HEXUPPER.decode(&hash.as_bytes()).unwrap(),
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Incorrect password.")),
    }
}

pub fn encrypt(password: String) -> (String, String) {
    let rng = ring_rand::SystemRandom::new();
    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).unwrap();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];

    pbkdf2::derive(
        &digest::SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut hash,
    );

    (HEXUPPER.encode(&hash), HEXUPPER.encode(&salt))
}
