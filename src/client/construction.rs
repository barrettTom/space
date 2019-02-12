extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::constants;
use crate::modules::construction::ConstructionStatus;
use crate::server::construction::ConstructionData;

pub fn client_construction(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data: ConstructionData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1, 1);

        if data.has_enough {
            match data.status {
                ConstructionStatus::None => {
                    write!(stdout, "{}Press c to create a refinery.", clear).unwrap()
                }
                _ => write!(stdout, "{}Press c to cancel..", clear).unwrap(),
            }
        } else {
            write!(
                stdout,
                "{}You need {} iron to create a refinery.",
                clear,
                constants::SHIP_CONSTRUCTION_IRON_COST
            )
            .unwrap();
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
