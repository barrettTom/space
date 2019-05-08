extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::mass::Mass;

pub fn client_dashboard(mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let ship: Result<Mass, serde_json::Error> = serde_json::from_str(&recv);
        if ship.is_err() {
            print!("{}", recv);
            break;
        }

        write!(
            stdout,
            "{}{}{:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            ship
        )
        .unwrap();

        let mut key = String::new();
        stdin.read_to_string(&mut key).unwrap();
        if key.as_str() == "q" {
            break;
        }

        stdout.flush().unwrap();
    }
}
