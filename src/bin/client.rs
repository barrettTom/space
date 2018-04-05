use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

extern crate serde_json;

extern crate space;
use space::dashboard::client_dashboard;
use space::engines::client_engines;
use space::navigation::client_navigation;
use space::module::Module;

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

    let mut data = String::new();
    buff_r.read_line(&mut data).unwrap();
    let modules : Vec<Module> = serde_json::from_str(&data.replace("\n","")).unwrap();

    println!("Choose your module:");
    for (i, module) in modules.iter().enumerate() {
        println!("{}) {:?}", i, module);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed");
    let module = modules[choice.replace("\n", "").parse::<usize>().unwrap()].clone();

    let send = serde_json::to_string(&module).unwrap() + "\n";
    stream.write(send.as_bytes()).unwrap();

    match module {
        Module::Dashboard => client_dashboard(buff_r),
        Module::Engines => client_engines(stream, buff_r),
        Module::Navigation => client_navigation(name, stream, buff_r),
    }
}
