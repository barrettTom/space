use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::time::SystemTime;

use crate::db::User;
use crate::db::{encrypt, verify};

use crate::schema::users::dsl as users_dsl;
use crate::schema::users::dsl::users as users_db;

#[derive(Deserialize)]
pub struct ControlPanel {
    pub ship_name: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn verify(&self, connection: &PgConnection) -> Result<(), String> {
        match users_db
            .filter(users_dsl::username.eq(self.username.clone()))
            .load::<User>(connection)
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
    pub username: String,
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
                username: self.username.clone(),
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
