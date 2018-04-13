extern crate serde_json;

use std::io::Write;
use std::collections::HashMap;

use mass::Mass;
use connection::Connection;

impl Connection {
    pub fn server_dashboard(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let ship = masses.get(&self.name).unwrap();
        let send = serde_json::to_string(&ship).unwrap();
        match self.stream.write(send.as_bytes()) {
            Ok(_result) => true,
            Err(_error) => false,
        }
    }
}
