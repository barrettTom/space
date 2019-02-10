extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::modules::tractorbeam::TractorbeamStatus;
use crate::server::tractorbeam::TractorbeamData;

pub fn client_tractorbeam(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data: TractorbeamData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1, 1);

        if data.has_target {
            match data.status {
                TractorbeamStatus::None => write!(
                    stdout,
                    "{}Press o to pull, p to push, t to bring to 5m.",
                    clear
                )
                .unwrap(),
                TractorbeamStatus::Push => write!(
                    stdout,
                    "{}Press o to pull, p to stop pushing, t to bring to 5m.",
                    clear
                )
                .unwrap(),
                TractorbeamStatus::Pull => write!(
                    stdout,
                    "{}Press o to stop pulling, p to push, t to bring to 5m.",
                    clear
                )
                .unwrap(),
                TractorbeamStatus::Bring => write!(
                    stdout,
                    "{}Press o to pulling, p to push, t to stop bringing to 5m.",
                    clear
                )
                .unwrap(),
            };
        } else {
            write!(stdout, "{}You have no target.", clear).unwrap();
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
