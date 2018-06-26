extern crate termion;
extern crate serde_json;

use std::net::TcpStream;
use self::termion::async_stdin;
use std::io::{BufReader, BufRead};
use std::io::{stdout, Read, Write};
use self::termion::raw::IntoRawMode;

use server::mining::MiningData;
use modules::mining::MiningStatus;

pub fn client_mining(mut stream : TcpStream, mut buff_r : BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data : MiningData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1,1);

        match data.has_astroid_target {
            true => match data.is_within_range {
                true => match data.astroid_has_minerals {
                    true => match data.status {
                            MiningStatus::None => write!(stdout, "{}Press F to begin mining.", clear).unwrap(),
                            _ => write!(stdout, "{}Press F to stop mining.", clear).unwrap(),
                    },
                    false => write!(stdout, "{}Astroid has ran out of minerals.", clear).unwrap(),
                }
                false => write!(stdout, "{}Astroid must be within range of {}.", clear, data.range).unwrap(),
            },
            false => write!(stdout, "{}Ship has no astroid targeted.", clear).unwrap(),
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
