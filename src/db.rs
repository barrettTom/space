use data_encoding::HEXUPPER;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand as ring_rand};
use std::num::NonZeroU32;
use std::time::SystemTime;

use crate::constants;
use crate::mass::Mass;
use crate::schema::masses as db_masses;
use crate::schema::users as db_users;
use crate::schema::users::dsl as users_dsl;
use crate::schema::users::dsl::users;

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Associations, Debug)]
#[belongs_to(User)]
#[table_name = "db_masses"]
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
}

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Debug)]
#[table_name = "db_users"]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub hash: String,
    pub salt: String,
    pub email: String,
    pub created: SystemTime,
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
        match users
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

    pub fn insert_into(
        &self,
        connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), String> {
        match diesel::insert_into(users)
            .values(&self.to_user()?)
            .execute(&connection)
        {
            Ok(_) => Ok(()),
            Err(_error) => Err(String::from("Already exists.")),
        }
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
