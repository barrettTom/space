extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, Read, Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use crate::modules::engines;

pub fn client_engines(mut stream: TcpStream, mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let engines_data: engines::ClientData =
            serde_json::from_str(&recv.replace("\n", "")).unwrap();

        writeln!(
            stdout,
            "{}{}Fuel: {:.2}\nuse numpad to freely move",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            engines_data.fuel
        )
        .unwrap();
        write!(stdout, "{}+ : speedup", termion::cursor::Goto(1, 2)).unwrap();
        write!(stdout, "{}- : slowdown", termion::cursor::Goto(1, 3)).unwrap();
        write!(stdout, "{}s : stop", termion::cursor::Goto(1, 4)).unwrap();
        write!(stdout, "{}q : quit", termion::cursor::Goto(1, 5)).unwrap();

        if engines_data.has_target {
            write!(
                stdout,
                "{}c : mimic targets velocity vector",
                termion::cursor::Goto(1, 6)
            )
            .unwrap();
            write!(
                stdout,
                "{}t : accelerate torwards target",
                termion::cursor::Goto(1, 7)
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
        sleep(Duration::from_millis(100));
    }
}
