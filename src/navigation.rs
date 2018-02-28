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

pub fn client_navigation(name : String, mut stream : TcpStream, mut buff_r : BufReader<TcpStream>){
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();

        let string_masses = data.split(";");
        let mut masses : Vec<Box<Mass>> = Vec::new();
        for string_mass in string_masses {
            if string_mass.len() == 1 {
                break;
            }
            masses.push(build_mass(string_mass));
        }

        let ship = masses.iter().find(|ship| ship.name() == &name);

        write!(stdout, "{}{}Targets:",
               termion::clear::All,
               termion::cursor::Goto(1,1)).unwrap();

        let position = ship.unwrap().position();
        for (i, mass) in masses.iter().enumerate() {
            write!(stdout, "{}{}) {} ({:.2}, {:.2}, {:.2}) Distance : {:.2}",
                   termion::cursor::Goto(1, 2 + i as u16),
                   i,
                   mass.name(),
                   mass.position().0,
                   mass.position().1,
                   mass.position().2,
                   distance(mass.position(), position)).unwrap();
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

pub fn server_navigation(masses : &mut Vec<Box<Mass>>, index : usize, mut stream : &TcpStream, buff_r : &mut BufReader<TcpStream>) -> bool {
    let ship = masses[index].downcast_ref::<Ship>().unwrap();

    let within_range : Vec<&Box<Mass>> = masses.iter().filter(|mass|
    distance(ship.position(), mass.position()) < ship.range()).collect();

    let mut send = String::new();
    for mass in within_range {
        send.push_str(&mass.serialize());
        send.push_str(";");
    }
    send.push_str("\n");
    match stream.write(send.as_bytes()) {
        Ok(_result) => (),
        Err(_error) => return false,
    }

    let mut string_mass = String::new();
    buff_r.read_line(&mut string_mass).unwrap();
    if string_mass.len() > 0 {
        let target = build_mass(&string_mass);
        //ship.give_target(masses.iter().position(|mass| 
        //                                        mass.name() == target.name()));
    }

    true
}

fn build_mass(string_mass : &str) -> Box<Mass> {
    let mut mass = Astroid::new();
    if string_mass.contains("Ship") {
        let mass : Ship = serde_json::from_str(&string_mass).unwrap();
    }
    else {
        let mass : Astroid = serde_json::from_str(&string_mass).unwrap();
    }
    Box::new(mass)
}
