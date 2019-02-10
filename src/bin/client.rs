extern crate clap;
extern crate serde_json;
extern crate space;
extern crate toml;

#[macro_use]
extern crate serde_derive;

use clap::{App, SubCommand};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

use space::client::construction::client_construction;
use space::client::dashboard::client_dashboard;
use space::client::engines::client_engines;
use space::client::mining::client_mining;
use space::client::navigation::client_navigation;
use space::client::refinery::client_refinery;
use space::client::tractorbeam::client_tractorbeam;
use space::modules::types::ModuleType;

#[derive(Debug, Deserialize)]
struct Config {
    username: Option<String>,
    password: Option<String>,
    server: Option<String>,
}

fn main() {
    let send;
    let server;
    let mut name = String::new();

    let matches = App::new("space")
        .subcommand(SubCommand::with_name("mining"))
        .subcommand(SubCommand::with_name("engines"))
        .subcommand(SubCommand::with_name("refinery"))
        .subcommand(SubCommand::with_name("dashboard"))
        .subcommand(SubCommand::with_name("navigation"))
        .subcommand(SubCommand::with_name("tractorbeam"))
        .subcommand(SubCommand::with_name("construction"))
        .get_matches();

    match File::open(".space") {
        Ok(mut config_file) => {
            let mut config_string = String::new();
            config_file.read_to_string(&mut config_string).unwrap();
            let config: Config = toml::from_str(&config_string).unwrap();

            server = config.server.unwrap();
            name = config.username.unwrap();
            send = name.clone() + ":" + &config.password.unwrap() + "\n";
        }
        Err(_err) => {
            println!("Ship Name:");
            io::stdin().read_line(&mut name).expect("Failed");
            name = name.replace("\n", "");

            let mut password = String::new();
            println!("Password:");
            io::stdin().read_line(&mut password).expect("Failed");

            server = "localhost:6000".to_string();
            send = name.clone() + ":" + &password;
        }
    }

    let mut stream = TcpStream::connect(&server).unwrap();
    let mut buff_r = BufReader::new(stream.try_clone().unwrap());

    stream.write_all(send.as_bytes()).unwrap();

    let mut recv = String::new();
    buff_r.read_line(&mut recv).unwrap();
    let modules: Vec<ModuleType> = serde_json::from_str(&recv.replace("\n", "")).unwrap();

    let module_type = match matches.subcommand_name() {
        Some("mining") => ModuleType::Mining,
        Some("engines") => ModuleType::Engines,
        Some("refinery") => ModuleType::Refinery,
        Some("dashboard") => ModuleType::Dashboard,
        Some("navigation") => ModuleType::Navigation,
        Some("tractorbeam") => ModuleType::Tractorbeam,
        Some("construction") => ModuleType::Construction,
        _ => {
            println!("Choose your module:");
            for (i, module) in modules.iter().enumerate() {
                println!("{}) {:?}", i, module);
            }

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed");
            modules
                .into_iter()
                .nth(choice.replace("\n", "").parse::<usize>().unwrap())
                .unwrap()
        }
    };

    let send = serde_json::to_string(&module_type).unwrap() + "\n";
    stream.write_all(send.as_bytes()).unwrap();

    match module_type {
        ModuleType::Dashboard => client_dashboard(buff_r),
        ModuleType::Mining => client_mining(stream, buff_r),
        ModuleType::Engines => client_engines(stream, buff_r),
        ModuleType::Refinery => client_refinery(stream, buff_r),
        ModuleType::Tractorbeam => client_tractorbeam(stream, buff_r),
        ModuleType::Construction => client_construction(stream, buff_r),
        ModuleType::Navigation => client_navigation(name, stream, buff_r),
    }
}
