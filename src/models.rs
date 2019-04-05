use std::time::SystemTime;

use crate::mass::Mass;
use crate::schema::masses as db_masses;
use crate::schema::users as db_users;

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
    pub password: String,
    pub last_modified: SystemTime,
}
