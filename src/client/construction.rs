extern crate termion;
extern crate serde_json;

use std::net::TcpStream;
use self::termion::async_stdin;
use std::io::{BufReader, BufRead};
use std::io::{stdout, Read, Write};
use self::termion::raw::IntoRawMode;

use server::construction::ConstructionData;
use modules::construction::ConstructionStatus;

pub fn client_construction(mut stream : TcpStream, mut buff_r : BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data : ConstructionData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1,1);

        match data.has_refined {
            true => match data.status {
                    ConstructionStatus::None => write!(stdout, "{}Press c to create a refinery.", clear).unwrap(),
                    _ => write!(stdout, "{}Press c to cancel..", clear).unwrap(),
            },
            false => write!(stdout, "{}You need 5 refined minerals to create a refinery.", clear).unwrap(),
        }

        match stdin.next() {
            Some(c) => {
                let c = c.unwrap();
                let mut send = String::new();
                send.push(c as char);
                if send.as_bytes() == b"q" {
                    break;
                }
                send.push_str("\n");
                stream.write(send.as_bytes()).unwrap();
            }
            None => ()
        }

        stdout.flush().unwrap();
    }
}
