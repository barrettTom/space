use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{stdout, Read, Write};
use termion::raw::IntoRawMode;
use termion::async_stdin;

extern crate serde_json;
extern crate termion;

use ship::Ship;
use math::distance;
use mass::{Mass, MassType};
use connection::Connection;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ServerData {
    has_astroid_target  : bool,
    is_within_range     : bool,
    mining_range        : f64,
    mining_status       : bool,
}

pub fn client_mining(mut stream : TcpStream, mut buff_r : BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let data : ServerData = serde_json::from_str(&recv.replace("\n", "")).unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();

        match data.has_astroid_target {
            true => match data.is_within_range {
                true => write!(stdout, "{}Press F to begin mining.", termion::cursor::Goto(1,1)).unwrap(),
                false => write!(stdout, "{}Astroid must be within range of {}.", termion::cursor::Goto(1,1), data.mining_range).unwrap(),
            },
            false => write!(stdout, "{}Ship has no astroid targeted.", termion::cursor::Goto(1,1)).unwrap(),
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

impl Connection {
    pub fn server_mining(&mut self, masses : &mut HashMap<String, Box<Mass>>) -> bool {
        let masses_clone = masses.clone();
        let mass = masses.get_mut(&self.name).unwrap();
        let ship = mass.downcast_mut::<Ship>().unwrap();

        let target = match ship.recv_target() {
            Some(name) => masses_clone.get(&name),
            None => None,
        };

        let has_astroid_target = match target {
            Some(target) => match target.recv_mass_type() {
                MassType::Ship => false,
                MassType::Astroid => true,
            },
            None => false,
        };

        let is_within_range = match has_astroid_target {
            true => match target {
                Some(target) => match ship.recv_mining_range() > distance(ship.position(), target.position()) {
                    true => true,
                    false => false,
                },
                None => false,
            }
            false => false,
        };

        let send = serde_json::to_string(&ServerData {
                                            has_astroid_target  : has_astroid_target,
                                            is_within_range     : is_within_range,
                                            mining_range        : ship.recv_mining_range(),
                                            mining_status       : ship.recv_mining_status(),
                                         }).unwrap() + "\n";

        match self.stream.write(send.as_bytes()) {
            Ok(_result) => (),
            Err(_error) => return false,
        }

        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(result) => match recv.as_bytes() {
                b"F\n" => {
                    if is_within_range {
                        match ship.recv_mining_status() {
                            true => ship.stop_mining(),
                            false => ship.start_mining(),
                        }
                    }
                },
                _ => {
                    if result == 0 {
                        return false
                    }
                },
            }
            Err(_error) => (),
        }

        true
    }
}
