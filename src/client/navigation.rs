extern crate termion;
//extern crate itertools;
extern crate serde_json;

use std::net::TcpStream;
use std::collections::BTreeMap;
use self::termion::async_stdin;
use std::io::{BufRead, BufReader};
use std::io::{stdout, Read, Write};
use self::termion::raw::IntoRawMode;

use math::distance;
use mass::{Mass, MassType};

pub fn client_navigation(name : String, mut stream : TcpStream, mut buff_r : BufReader<TcpStream>){
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let mut within_range : BTreeMap<String, Mass> = serde_json::from_str(&recv).unwrap();

        write!(stdout, "{}{}Targets:",
               termion::clear::All,
               termion::cursor::Goto(1,1)).unwrap();

        let ship = within_range.remove(&name).unwrap();
        let targeting = match ship.mass_type {
            MassType::Ship{ref targeting, ..} => Some(targeting.clone()),
            _ => None,
        }.unwrap();

        for (i, (mass_name, mass)) in within_range.iter().enumerate() {

            let target_data = match targeting.target.clone() {
                Some(target_name) => {
                    if &target_name == mass_name {
                        serde_json::to_string(&targeting.status).unwrap()
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
                   mass.position.0,
                   mass.position.1,
                   mass.position.2,
                   distance(mass.position, ship.position),
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
                    if i < within_range.len() {
                        let mut send = String::new();
                        send.push_str(within_range.iter().nth(i).unwrap().0);
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
