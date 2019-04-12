use diesel::pg::PgConnection;
use diesel::prelude::*;

use space::db::get_db_url;

fn main() {
    let connection = PgConnection::establish(&get_db_url()).expect("Cannot connect");
    migrations_internals::run_pending_migrations(&connection).expect("Cannot run migrations");
}
