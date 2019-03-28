use diesel::pg::PgConnection;
use diesel::prelude::*;

use space::math::get_db_url;

fn main() {
    let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");
    migrations_internals::revert_latest_migration(&connection).expect("Cannot revert migrations");
    migrations_internals::revert_latest_migration(&connection).expect("Cannot revert migrations");
}
