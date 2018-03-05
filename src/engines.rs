use std::net::TcpStream;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::thread::sleep;
use std::io::{Read, Write, stdout};
use std::time::Duration;
use std::io::BufRead;

use ship::Ship;
use mass::Mass;
use connection::Connection;

pub fn client_engines(mut stream : TcpStream) {
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
                stream.write(send.as_bytes()).unwrap();
            }
            None => ()
        }

        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }
}

impl Connection {
    pub fn server_engines(&mut self, masses : &mut Vec<Box<Mass>>) -> bool {
        let ship = masses[self.index].downcast_mut::<Ship>().unwrap();
        let mut acceleration = (0.0, 0.0, 0.0);
        let mut data = String::new();
        match self.buff_r.read_line(&mut data) {
            Ok(result) => match data.as_bytes() {
                b"5\n" => acceleration.0 += 0.1,
                b"0\n" => acceleration.0 -= 0.1,
                b"8\n" => acceleration.1 += 0.1,
                b"2\n" => acceleration.1 -= 0.1,
                b"4\n" => acceleration.2 += 0.1,
                b"6\n" => acceleration.2 -= 0.1,
                b"-\n" => ship.slow(),
                _ => {
                    if result == 0 {
                        return false
                    }
                },
            },
            Err(_error) => (),
        }
        ship.give_acceleration(acceleration);
        true
    }
}
