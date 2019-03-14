extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::modules::tractorbeam;

pub fn client_tractorbeam(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data: tractorbeam::ClientData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1, 1);

        if data.has_target {
            match data.status {
                tractorbeam::Status::None => write!(
                    stdout,
                    "{}Press o to pull, p to push, b to bring to 5m.",
                    clear
                )
                .unwrap(),
                tractorbeam::Status::Push => write!(
                    stdout,
                    "{}Press o to pull, p to stop pushing, b to bring to 5m.",
                    clear
                )
                .unwrap(),
                tractorbeam::Status::Pull => write!(
                    stdout,
                    "{}Press o to stop pulling, p to push, b to bring to 5m.",
                    clear
                )
                .unwrap(),
                tractorbeam::Status::Bring => write!(
                    stdout,
                    "{}Press o to pulling, p to push, b to stop bringing to 5m.",
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
