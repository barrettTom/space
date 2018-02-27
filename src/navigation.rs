use std::net::TcpStream;
use std::io::{BufRead, BufReader};

use std::io::{stdout, Read, Write, stdin};
use termion::raw::IntoRawMode;
use termion::async_stdin;

extern crate erased_serde;
extern crate serde_json;
extern crate termion;

use erased_serde::Deserializer;

use mass::Mass;
use ship::Ship;
use math::distance;
use astroid::Astroid;

pub fn Navigation(name : String, mut stream :TcpStream, mut buff_r : BufReader<TcpStream>){
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();

        let string_masses = data.split(";");
        let mut masses : Vec<Box<Mass>> = Vec::new();
        let mut ship : Option<Ship> = None;
        for string_mass in string_masses {
            if string_mass.len() == 1 {
                break;
            }
            let json = &mut serde_json::de::Deserializer::from_slice(string_mass.as_bytes());
            let mut deserialized : Box<Deserializer> = Box::new(Deserializer::erase(json));

            if string_mass.contains("Ship") {
                let mass : Ship = erased_serde::deserialize(&mut deserialized).unwrap();
                if mass.name() == &name {
                    ship = Some(mass);
                }
                else {
                    masses.push(Box::new(mass));
                }
            }
            else {
                let mass : Astroid = erased_serde::deserialize(&mut deserialized).unwrap();
                masses.push(Box::new(mass));
            }
        }


        write!(stdout, "{}{}Targets:",
               termion::clear::All,
               termion::cursor::Goto(1,1)).unwrap();

        let location = ship.expect("zz").location();
        for (i, mass) in masses.iter().enumerate() {
            write!(stdout, "{}{}) {} {:?} Distance : {}",
                   termion::cursor::Goto(1, 2 + i as u16),
                   i,
                   mass.name(),
                   mass.location(),
                   distance(mass.location(), location)).unwrap();
        }

        match stdin.next() {
            Some(c) => {
                let c = c.unwrap();
                let mut send = String::new();
                send.push(c as char);
                if send.as_bytes() == b"q" {
                    break;
                }
                //send.push_str("\n");
                //stream.write(send.as_bytes()).unwrap();
            }
            None => ()
        }

        stdout.flush().unwrap();
    }
}
