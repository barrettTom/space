use data_encoding::HEXUPPER;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand as ring_rand};
use std::num::NonZeroU32;
use std::time::SystemTime;

use crate::constants;
use crate::mass::Mass;

use crate::schema::masses as masses_schema;
use crate::schema::masses::dsl as masses_dsl;
use crate::schema::masses::dsl::masses as masses_db;

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

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Debug)]
#[table_name = "users_schema"]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub hash: String,
    pub salt: String,
    pub email: String,
    pub created: SystemTime,
}

impl User {
    pub fn get(
        username: String,
        connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> User {
        users_db
            .filter(users_dsl::username.eq(username))
            .load::<User>(&connection)
            .expect("Cannot get")
            .pop()
            .unwrap()
    }

    pub fn get_ship(&self, connection: &PgConnection) -> Option<Mass> {
        match masses_db
            .filter(masses_dsl::user_id.eq(self.id))
            .load::<MassEntry>(connection)
            .expect("Cannot get")
            .pop()
        {
            Some(mass_entry) => Some(mass_entry.to_mass().1),
            None => None,
        }
    }

    pub fn get_ship_name(&self, connection: &PgConnection) -> Option<String> {
        match masses_db
            .filter(masses_dsl::user_id.eq(self.id))
            .load::<MassEntry>(connection)
            .expect("Cannot get")
            .pop()
        {
            Some(mass_entry) => Some(mass_entry.to_mass().0),
            None => None,
        }
    }

    pub fn give_ship(&self, ship_name: String, connection: &PgConnection) {
        Mass::new_ship()
            .to_mass_entry(ship_name, self.id, SystemTime::now())
            .insert_into(connection);
    }

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
        diesel::delete(users_db.filter(users_dsl::username.eq(self.username.clone())))
            .execute(&connection)
            .expect("Cannot delete.");
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
