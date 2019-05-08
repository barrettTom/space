extern crate diesel;
extern crate migrations_internals;
extern crate space;

use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::time::SystemTime;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use space::constants;
use space::db::{encrypt, get_db_url, verify};
use space::forms::{Login, Registration};
use space::mass::Mass;
use space::masses::{Init, Masses};
use space::modules::types::ModuleType;
use space::server_connection::ServerConnection;

fn setup() -> (Mass, Masses) {
    (Mass::new_ship(), Masses::new(Init::Test))
}

#[test]
fn test_db() {
    test_postgres();
    test_register_login_user();
    test_server_connection();
}

fn test_postgres() {
    let masses = Masses::new(Init::None);
    let len_before = masses.hashmap.len();

    let name = String::from("test_postgres");
    let mut mass = Mass::new_astroid();

    masses.insert(mass.to_mass_entry(name.clone(), None, SystemTime::now()));

    assert!(masses.len() == len_before + 1);

    let db_mass = masses.get(name.clone());

    assert!((mass.position.x - db_mass.to_mass().1.position.x).abs() < constants::FLOAT_PRECISION);

    mass.process(&mut HashMap::new());

    masses.update(mass.to_mass_entry(name.clone(), None, SystemTime::now()));

    let db_mass = masses.get(name.clone());

    assert!((mass.position.x - db_mass.to_mass().1.position.x).abs() < constants::FLOAT_PRECISION);

    masses.delete(db_mass);

    assert!(masses.hashmap.len() == len_before);
}

fn test_register_login_user() {
    let pool = Pool::new(ConnectionManager::<PgConnection>::new(get_db_url())).unwrap();
    let username = String::from("test_register_login_user");
    let pass = String::from("test");
    let user = Registration {
        username: username.clone(),
        email: "test_register_login_user@test.com".to_string(),
        password1: pass.clone(),
        password2: pass.clone(),
    }
    .to_user()
    .unwrap();

    user.insert_into(pool.get().unwrap()).unwrap();

    let mut form = Login {
        username: username.clone(),
        password: pass.clone(),
    };
    assert!(form.verify(&pool.get().unwrap()).is_ok());

    form.password.push_str("zz");
    assert!(!form.verify(&pool.get().unwrap()).is_ok());

    form.username.push_str("zz");
    assert!(!form.verify(&pool.get().unwrap()).is_ok());

    user.delete(pool.get().unwrap());
}

fn test_server_connection() {
    let (_, mut masses) = setup();

    let listener = TcpListener::bind(constants::SERVER_IP_PORT).unwrap();
    let mut client_stream = TcpStream::connect(constants::SERVER_IP_PORT).unwrap();

    let username = String::from("test_server_connection");
    let ship_name = String::from("test_server_connection_ship");
    let password = String::from("pass");

    let send = username.clone()
        + ":"
        + &ship_name
        + ":"
        + &password
        + ":"
        + &serde_json::to_string(&ModuleType::Mining).unwrap()
        + "\n";

    client_stream.write_all(send.as_bytes()).unwrap();
    if let Ok((stream, _)) = listener.accept() {
        assert!(ServerConnection::new(stream, &mut masses).is_err());
    }

    let pool = Pool::new(ConnectionManager::<PgConnection>::new(get_db_url())).unwrap();

    let user = Registration {
        username: username.clone(),
        email: "test_server_connection@test.com".to_string(),
        password1: password.clone(),
        password2: password.clone(),
    }
    .to_user()
    .unwrap();

    user.insert_into(pool.get().unwrap()).unwrap();

    user.give_ship(ship_name.clone(), &pool.get().unwrap());
    masses.import();

    let mut client_stream = TcpStream::connect(constants::SERVER_IP_PORT).unwrap();
    client_stream.write_all(send.as_bytes()).unwrap();
    if let Ok((stream, _)) = listener.accept() {
        assert!(ServerConnection::new(stream, &mut masses).is_ok());
    }

    user.delete(pool.get().unwrap());
    masses.get(ship_name).delete(&pool.get().unwrap());
}

#[test]
fn test_salt_hash() {
    let mut password = String::from("this is a test password");
    let (hash, salt) = encrypt(password.clone());
    assert!(verify(password.clone(), hash.clone(), salt.clone()).is_ok());
    password.push_str("bad");
    assert!(!verify(password, hash, salt).is_ok());
}
