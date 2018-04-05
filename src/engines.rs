use std::net::TcpStream;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::thread::sleep;
use std::io::{Read, Write, stdout};
use std::time::Duration;
use std::io::{BufRead, BufReader};

extern crate termion;

use ship::Ship;
use mass::Mass;
use connection::Connection;

pub fn client_engines(mut stream : TcpStream, mut buff_r : BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();

        let has_target = match recv.as_bytes() {
            b"true\n" => true,
            _ => false
        };

        write!(stdout, "{}{}use numpad to freely move\n", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        write!(stdout, "{}+ : speedup", termion::cursor::Goto(1, 2)).unwrap();
        write!(stdout, "{}- : slowdown", termion::cursor::Goto(1, 3)).unwrap();
        write!(stdout, "{}q : quit", termion::cursor::Goto(1, 4)).unwrap();

        if has_target {
            write!(stdout, "{}c : mimic targets velocity vector", termion::cursor::Goto(1,5)).unwrap();
            write!(stdout, "{}t : accelerate torwards target", termion::cursor::Goto(1,6)).unwrap();
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
        sleep(Duration::from_millis(100));
    }
}

impl Connection {
    pub fn server_engines(&mut self, masses : &mut Vec<Box<Mass>>) -> bool {
        let m = masses.to_vec();
        let mass = masses.into_iter().find(|ship| ship.name() == &self.name).unwrap();
        let ship = mass.downcast_mut::<Ship>().unwrap();

        let mut send = String::new();
        match ship.recv_target().is_some() {
            true => send.push_str("true\n"),
            false => send.push_str("false\n"),
        }
        match self.stream.write(send.as_bytes()) {
            Ok(_result) => (),
            Err(_error) => return false,
        }

        let mut acceleration = (0.0, 0.0, 0.0);
        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(result) => match recv.as_bytes() {
                b"5\n" => acceleration.0 += 0.1,
                b"0\n" => acceleration.0 -= 0.1,
                b"8\n" => acceleration.1 += 0.1,
                b"2\n" => acceleration.1 -= 0.1,
                b"4\n" => acceleration.2 += 0.1,
                b"6\n" => acceleration.2 -= 0.1,
                b"+\n" => ship.speedup(),
                b"-\n" => ship.slow(),
                b"c\n" => {
                    match ship.recv_target() {
                        Some(name) => {
                            let target = m.into_iter().find(|target| target.name() == &name).unwrap();
                            let d_v = target.recv_velocity();
                            let m_v = ship.recv_velocity();
                            acceleration = (d_v.0 - m_v.0,
                                            d_v.1 - m_v.1,
                                            d_v.2 - m_v.2);
                        },
                        None => (),
                    }
                },
                b"t\n" => {
                    match ship.recv_target() {
                        Some(name) => {
                            let target = m.into_iter().find(|target| target.name() == &name).unwrap();
                            let d_p = target.recv_velocity();
                            let m_p = ship.position();
                            acceleration = ((d_p.0 - m_p.0) * 0.01,
                                            (d_p.1 - m_p.1) * 0.01,
                                            (d_p.2 - m_p.2) * 0.01);
                        },
                        None => (),
                    }
                },
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
