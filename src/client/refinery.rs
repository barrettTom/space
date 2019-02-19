extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::modules::refinery::{RefineryClientData, RefineryStatus};

pub fn client_refinery(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data: RefineryClientData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1, 1);

        if data.has_crude_minerals {
            match data.status {
                RefineryStatus::None => {
                    write!(stdout, "{}Press R to begin refining.", clear).unwrap()
                }
                _ => write!(stdout, "{}Press R to stop refining.", clear).unwrap(),
            };
        } else {
            write!(stdout, "{}You have no crude minerals.", clear).unwrap();
        }

        if let Some(c) = stdin.next() {
            let c = c.unwrap();
            let mut send = String::new();
            send.push(c as char);
            if send.as_bytes() == b"q" {
                break;
            }
            send.push_str("\n");
            stream.write_all(send.as_bytes()).unwrap();
        }

        stdout.flush().unwrap();
    }
}
