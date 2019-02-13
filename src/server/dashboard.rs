extern crate serde_json;

use std::collections::HashMap;
use std::io::Write;

use crate::mass::Mass;
use crate::server::connection::ServerConnection;

impl ServerConnection {
    pub fn server_dashboard(&mut self, masses: &mut HashMap<String, Mass>) {
        if self.open {
            let ship = masses.get(&self.name).unwrap();
            let send = serde_json::to_string(&ship).unwrap() + "\n";
            self.open = self.stream.write(send.as_bytes()).is_ok();
        }
    }
}
