use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use std::io::{stdout, Read, Write};
use termion::raw::IntoRawMode;
use termion::async_stdin;

extern crate serde_json;
extern crate termion;

use mass::Mass;
use ship::Ship;
use math::distance;
use astroid::Astroid;
use connection::Connection;

pub fn client_navigation(name : String, mut stream : TcpStream, mut buff_r : BufReader<TcpStream>){
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();

        let string_masses = recv.split(";");
        let mut masses : Vec<Box<Mass>> = Vec::new();
        for string_mass in string_masses {
            if string_mass.len() <= 1 {
                break;
            }
            masses.push(build_mass(string_mass));
        }

        let index = masses.iter().position(|ship| ship.name() == &name).unwrap();
        let ship = masses.remove(index).downcast::<Ship>().unwrap();

        write!(stdout, "{}{}Targets:",
               termion::clear::All,
               termion::cursor::Goto(1,1)).unwrap();

        for (i, mass) in masses.iter().enumerate() {

            let target_data = match ship.recv_target() {
                Some(name) => {
                    if &name == mass.name() {
                        serde_json::to_string(&ship.recv_target_status()).unwrap()
                    }
                    else {
                        String::new()
                    }
                }
                None => String::new(),
            };

            write!(stdout, "{}{}) {} ({:.2}, {:.2}, {:.2}) Distance : {:.2} {}",
                   termion::cursor::Goto(1, 2 + i as u16),
                   i,
                   mass.name(),
                   mass.position().0,
                   mass.position().1,
                   mass.position().2,
                   distance(mass.position(), ship.position()),
                   target_data
                   ).unwrap();
        }

        match stdin.next() {
            Some(c) => {
                let c = c.unwrap();
                let mut send = String::new();
                send.push(c as char);
                if send.as_bytes() == b"q" {
                    break;
                }
                else {
                    let i = match send.parse::<usize>() {
                        Ok(num) => num,
                        Err(_err) => 100,
                    };
                    if i < masses.len() {
                        send = masses[i].serialize();
                        send.push_str("\n");
                        stream.write(send.as_bytes()).unwrap();
                    }
                }
            }
            None => ()
        }

        stdout.flush().unwrap();
    }
}

impl Connection {
    pub fn server_navigation(&mut self, masses : &mut Vec<Box<Mass>>) -> bool {
        let m = masses.to_vec();
        let mass = masses.into_iter().find(|ship| ship.name() == &self.name).unwrap();
        let ship = mass.downcast_mut::<Ship>().unwrap();

        match ship.recv_target() {
            Some(name) => {
                let target = m.iter().find(|target| target.name() == &name).unwrap();
                if distance(target.position(), ship.position()) > ship.range() {
                    ship.give_target(None);
                }
            }
            None => (),
        }

        let within_range : Vec<&Box<Mass>> = m.iter().filter(|mass|
                                                             distance(ship.position(), mass.position()) < ship.range())
                                                             .collect();
        let mut send = String::new();
        for mass in within_range {
            send.push_str(&mass.serialize());
            send.push_str(";");
        }
        send.push_str("\n");
        match self.stream.write(send.as_bytes()) {
            Ok(_result) => (),
            Err(_error) => return false,
        }

        let mut string_mass = String::new();
        match self.buff_r.read_line(&mut string_mass) {
            Ok(_result) => (),
            Err(_error) => (),
        }
        if string_mass.len() > 0 {
            let target = build_mass(&string_mass);
            let name = target.name().clone();
            ship.give_target(Some(name));
        }
        true
    }
}

fn build_mass(string_mass : &str) -> Box<Mass> {
    if string_mass.contains("Ship") {
        let mass : Ship = serde_json::from_str(&string_mass).unwrap();
        return Box::new(mass)
    }
    else {
        let mass : Astroid = serde_json::from_str(&string_mass).unwrap();
        return Box::new(mass)
    }
}
