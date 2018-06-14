extern crate serde_json;

use std::io::Write;
use std::collections::HashMap;

use mass::Mass;
use server::connection::ServerConnection;

impl ServerConnection {
    pub fn server_dashboard(&mut self, masses : &mut HashMap<String, Mass>) {
        if self.open {
            let ship = masses.get(&self.name).unwrap();
            let send = serde_json::to_string(&ship).unwrap() + "\n";

            self.open = match self.stream.write(send.as_bytes()) {
                Ok(_result) => true,
                Err(_error) => false,
            };
        }
    }
}
