extern crate termion;
extern crate itertools;
extern crate serde_json;

use std::net::TcpStream;
use std::collections::BTreeMap;
use self::termion::async_stdin;
use self::itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::io::{stdout, Read, Write};
use self::termion::raw::IntoRawMode;

use mass::Mass;
use ship::Ship;
use math::distance;
use astroid::Astroid;

pub fn client_navigation(name : String, mut stream : TcpStream, mut buff_r : BufReader<TcpStream>){
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();

        let string_hashmap = recv.split(";");
        let mut masses : BTreeMap<String, Box<Mass>> = BTreeMap::new();
        for string_element in string_hashmap {
            if string_element.len() <= 1 {
                break;
            }
            let (string_name, string_mass) = string_element.split("@").next_tuple().unwrap();
            masses.insert(string_name.to_string(), build_mass(string_mass));
        }


        write!(stdout, "{}{}Targets:",
               termion::clear::All,
               termion::cursor::Goto(1,1)).unwrap();

        let ship = masses.remove(&name).unwrap().downcast::<Ship>().unwrap();

        for (i, (mass_name, mass)) in masses.iter().enumerate() {

            let target_data = match ship.recv_target() {
                Some(target_name) => {
                    if &target_name == mass_name {
                        serde_json::to_string(&ship.recv_targeting_status()).unwrap()
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
                   mass_name,
                   mass.position().0,
                   mass.position().1,
                   mass.position().2,
                   distance(mass.position(), ship.position()),
                   target_data
                   ).unwrap();
        }

        match stdin.next() {
            Some(c) => {
                let c = c.unwrap() as char;
                if c == 'q' {
                    break;
                }
                else {
                    let i = c.to_digit(10).unwrap() as usize;
                    if i < masses.len() {
                        let mut send = String::new();
                        send.push_str(masses.iter().nth(i).unwrap().0);
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
