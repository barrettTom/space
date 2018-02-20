use std::io;
use std::thread;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};

extern crate space;
use space::ship::Ship;

extern crate serde_json;

fn get_login_info() -> String {
    let mut name = String::new();
    println!("Ship Name:");
    io::stdin().read_line(&mut name).expect("Failed");

    let mut password = String::new();
    println!("Password:");
    io::stdin().read_line(&mut password).expect("Failed");

    name.replace("\n", "") + ":" + &password
}

fn main() {
    let mut stream = TcpStream::connect("localhost:6000").unwrap();
    let mut buff_r = BufReader::new(stream.try_clone().unwrap());

    let send = get_login_info();
    stream.write(send.as_bytes());

    let mut data = String::new();
    buff_r.read_line(&mut data);
    let modules : Vec<&str> = data.split(",").collect();

    println!("Choose your module:");
    for (i, module) in modules.iter().enumerate() {
        println!("{}) {}", i, module);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed");
    stream.write(choice.as_bytes());

    let mut data = String::new();
    buff_r.read_line(&mut data);
    let ship : Ship = serde_json::from_str(&data).unwrap();
    println!("{:?}", ship.location.0);

    stream.shutdown(Shutdown::Both);
}
