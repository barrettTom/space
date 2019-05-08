#[macro_use]
extern crate serde;

extern crate clap;
extern crate serde_json;
extern crate space;
extern crate toml;

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
    ship_name: Option<String>,
    password: Option<String>,
    server: Option<String>,
}

fn main() {
    let server;

    let matches = App::new("space client")
        .subcommand(SubCommand::with_name("mining"))
        .subcommand(SubCommand::with_name("engines"))
        .subcommand(SubCommand::with_name("refinery"))
        .subcommand(SubCommand::with_name("dashboard"))
        .subcommand(SubCommand::with_name("navigation"))
        .subcommand(SubCommand::with_name("tractorbeam"))
        .subcommand(SubCommand::with_name("construction"))
        .get_matches();

    let mut send = match File::open(".space") {
        Ok(mut config_file) => {
            let mut config_string = String::new();
            config_file.read_to_string(&mut config_string).unwrap();
            let config: Config = toml::from_str(&config_string).unwrap();

            server = config.server.unwrap();
            config.username.unwrap()
                + ":"
                + &config.ship_name.unwrap()
                + ":"
                + &config.password.unwrap()
                + ":"
        }
        Err(_err) => {
            let mut username = String::new();
            println!("Ship Name:");
            io::stdin().read_line(&mut username).expect("Failed");
            username = username.replace("\n", "");

            let mut ship_name = String::new();
            println!("Ship Name:");
            io::stdin().read_line(&mut ship_name).expect("Failed");
            ship_name = ship_name.replace("\n", "");

            let mut password = String::new();
            println!("Password:");
            io::stdin().read_line(&mut password).expect("Failed");
            password = password.replace("\n", "");

            server = "localhost:6000".to_string();
            username + ":" + &ship_name + ":" + &password + ":"
        }
    };

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
            for (i, module) in ModuleType::iter().iter().enumerate() {
                println!("{}) {:?}", i, module);
            }

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed");
            ModuleType::iter()
                .into_iter()
                .nth(choice.replace("\n", "").parse::<usize>().unwrap())
                .unwrap()
        }
    };

    send += &(serde_json::to_string(&module_type).unwrap() + "\n");

    let mut stream = TcpStream::connect(&server).unwrap();
    let buff_r = BufReader::new(stream.try_clone().unwrap());
    stream.write_all(send.as_bytes()).unwrap();

    match module_type {
        ModuleType::Dashboard => client_dashboard(buff_r),
        ModuleType::Mining => client_mining(stream, buff_r),
        ModuleType::Engines => client_engines(stream, buff_r),
        ModuleType::Refinery => client_refinery(stream, buff_r),
        ModuleType::Navigation => client_navigation(stream, buff_r),
        ModuleType::Tractorbeam => client_tractorbeam(stream, buff_r),
        ModuleType::Construction => client_construction(stream, buff_r),
    }
}
