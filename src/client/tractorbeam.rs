extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::FromStr;

use crate::modules::tractorbeam;

pub fn client_tractorbeam(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut async_in = async_stdin();

    let mut server_recv_data = tractorbeam::ServerRecvData {
        key: String::from_str("").unwrap(),
        desired_distance: None,
    };

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();

        let data: Result<tractorbeam::ClientData, serde_json::Error> = serde_json::from_str(&recv);
        if data.is_err() {
            print!("{}", recv);
            break;
        }
        let data = data.unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        let clear = termion::cursor::Goto(1, 1);
        if data.has_target {
            match data.status {
                tractorbeam::Status::None => write!(
                    stdout,
                    "{}Press o to pull, p to push, b to bring to a specific distance, a to acquire item.",
                    clear
                )
                .unwrap(),
                tractorbeam::Status::Push => write!(
                    stdout,
                    "{}Press o to pull, p to stop pushing, b to bring to a specific distance, a to acquire item.",
                    clear
                )
                .unwrap(),
                tractorbeam::Status::Pull => write!(
                    stdout,
                    "{}Press o to stop pulling, p to push, b to bring to a specific distance, a to acquire item.",
                    clear
                )
                .unwrap(),
                tractorbeam::Status::Bring => write!(
                    stdout,
                    "{}Press o to pull, p to push, b to stop bringing to {} m, a to acquire item.",
                    clear,
                    data.desired_distance.unwrap(),
                )
                .unwrap(),
                tractorbeam::Status::Acquire => write!(
                    stdout,
                    "{}Press o to pull, p to push, b to bring to a specific distance, a to stop acquiring the item.",
                    clear
                )
                .unwrap(),
            };
        } else {
            write!(stdout, "{}You have no target.", clear).unwrap();
        }

        server_recv_data.desired_distance = None;

        let mut key = String::new();
        async_in.read_to_string(&mut key).unwrap();

        if key.as_str() == "q" {
            break;
        } else if key.as_str() == "b" && data.has_target {
            server_recv_data.desired_distance = Some(5.0);
        }

        server_recv_data.key = key.to_string();

        let send = serde_json::to_string(&server_recv_data).unwrap() + "\n";
        stream.write_all(send.as_bytes()).unwrap();

        stdout.flush().unwrap();
    }
}
