extern crate termion;
extern crate serde_json;

use std::net::TcpStream;
use std::io::{BufRead, BufReader, stdout, Write, Read};
use self::termion::raw::IntoRawMode;
use self::termion::async_stdin;

use mass::Mass;

pub fn client_dashboard(mut buff_r : BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let ship : Mass = serde_json::from_str(&recv).unwrap();

        write!(stdout, "{}{}{:?}",
               termion::clear::All,
               termion::cursor::Goto(1,1),
               ship).unwrap();

        if let Some(c) = stdin.next() {
            let c  = c.unwrap() as char;
            if c == 'q' {
                break;
            }
        }
        stdout.flush().unwrap();
    }
}
