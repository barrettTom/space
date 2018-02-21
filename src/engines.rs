use std::net::TcpStream;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::thread::sleep;
use std::io::{Read, Write, stdout};
use std::time::Duration;

pub fn Engines(mut stream : TcpStream) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        match stdin.next() {
            Some(c) => {
                let c = c.unwrap();
                let mut send = String::new();
                send.push(c as char);
                if send.as_bytes() == b"q" {
                    break;
                }
                send.push_str("\n");
                stream.write(send.as_bytes());
            }
            None => ()
        }

        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }
}
