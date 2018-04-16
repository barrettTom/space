extern crate space;
extern crate serde_json;

use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::BTreeMap;

use space::module::{Module, ModuleType};
use space::client::mining::client_mining;
use space::client::engines::client_engines;
use space::client::dashboard::client_dashboard;
use space::client::navigation::client_navigation;

fn main() {
    let mut name = String::new();
    println!("Ship Name:");
    io::stdin().read_line(&mut name).expect("Failed");
    name = name.replace("\n", "");

    let mut password = String::new();
    println!("Password:");
    io::stdin().read_line(&mut password).expect("Failed");

    let send = name.clone() + ":" + &password;

    let mut stream = TcpStream::connect("localhost:6000").unwrap();
    let mut buff_r = BufReader::new(stream.try_clone().unwrap());

    stream.write(send.as_bytes()).unwrap();

    let mut recv = String::new();
    buff_r.read_line(&mut recv).unwrap();
    let modules : BTreeMap<String, Module> = serde_json::from_str(&recv.replace("\n","")).unwrap();

    println!("Choose your module:");
    for (i, module) in modules.keys().enumerate() {
        println!("{}) {:?}", i, module);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed");
    let module = modules.values().nth(choice.replace("\n", "").parse::<usize>().unwrap()).unwrap();

    let send = serde_json::to_string(&module).unwrap() + "\n";
    stream.write(send.as_bytes()).unwrap();

    match module.module_type {
        ModuleType::Dashboard => client_dashboard(buff_r),
        ModuleType::Engines => client_engines(stream, buff_r),
        ModuleType::Mining{..} => client_mining(stream, buff_r),
        ModuleType::Navigation{..} => client_navigation(name, stream, buff_r),
    }
}
