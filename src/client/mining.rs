extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::modules::mining::{MiningClientData, MiningStatus};

pub fn client_mining(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data: MiningClientData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1, 1);

        if data.has_astroid_target {
            if data.is_within_range {
                if data.astroid_has_minerals {
                    match data.status {
                        MiningStatus::None => {
                            write!(stdout, "{}Press F to begin mining.", clear).unwrap()
                        }
                        _ => write!(stdout, "{}Press F to stop mining.", clear).unwrap(),
                    };
                } else {
                    write!(stdout, "{}Astroid has ran out of minerals.", clear).unwrap();
                }
            } else {
                write!(
                    stdout,
                    "{}Astroid must be within range of {}.",
                    clear, data.range
                )
                .unwrap();
            }
        } else {
            write!(stdout, "{}Ship has no astroid targeted.", clear).unwrap();
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
