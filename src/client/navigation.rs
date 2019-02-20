extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::modules::navigation;

pub fn client_navigation(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let navigation_data: navigation::ClientData = serde_json::from_str(&recv).unwrap();

        write!(
            stdout,
            "{}{}Targets:",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();

        for (i, (name, position)) in navigation_data.available_targets.iter().enumerate() {
            let target_status = match &navigation_data.target_name {
                Some(target_name) => {
                    if target_name == name {
                        serde_json::to_string(&navigation_data.status).unwrap()
                    } else {
                        String::new()
                    }
                }
                None => String::new(),
            };
            write!(
                stdout,
                "{}{}) {} {} Distance : {:.2} {}",
                termion::cursor::Goto(1, 2 + i as u16),
                i,
                name,
                position,
                position.distance_from(navigation_data.ship_position.clone()),
                target_status
            )
            .unwrap();
        }

        if let Some(c) = stdin.next() {
            let c = c.unwrap() as char;
            if c == 'q' {
                break;
            } else {
                let i = c.to_digit(10).unwrap() as usize;
                if i < navigation_data.available_targets.len() {
                    let mut send = String::new();
                    send.push_str(&navigation_data.available_targets[i].0);
                    send.push_str("\n");
                    stream.write_all(send.as_bytes()).unwrap();
                }
            }
        }
        stdout.flush().unwrap();
    }
}
