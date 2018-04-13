extern crate serde_json;

use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;

use mass::{Mass, MassType};
use connection::Connection;
use targeting::TargetingStatus;

impl Connection {
    pub fn server_engines(&mut self, masses : &mut HashMap<String, Mass>) -> bool {
        let masses_clone = masses.clone();

        let ship = masses.get_mut(&self.name).unwrap();
        let targeting = match ship.mass_type {
            MassType::Ship{ref targeting, ..} => Some(targeting.clone()),
            _ => None,
        }.unwrap();

        let targeted = targeting.status == TargetingStatus::Targeted;
        let send = serde_json::to_string(&targeted).unwrap() + "\n";

        match self.stream.write(send.as_bytes()) {
            Ok(_result) => (),
            Err(_error) => return false,
        }

        let mut acceleration = (0.0, 0.0, 0.0);
        let mut recv = String::new();
        match self.buff_r.read_line(&mut recv) {
            Ok(result) => match recv.as_bytes() {
                b"5\n" => acceleration.0 += 0.1,
                b"0\n" => acceleration.0 -= 0.1,
                b"8\n" => acceleration.1 += 0.1,
                b"2\n" => acceleration.1 -= 0.1,
                b"4\n" => acceleration.2 += 0.1,
                b"6\n" => acceleration.2 -= 0.1,
                b"+\n" => {
                    let m_v = ship.velocity;
                    acceleration = (m_v.0 * 0.05,
                                    m_v.1 * 0.05,
                                    m_v.2 * 0.05);
                },
                b"-\n" => {
                    let m_v = ship.velocity;
                    acceleration = (-1.0 * m_v.0 * 0.05,
                                    -1.0 * m_v.1 * 0.05,
                                    -1.0 * m_v.2 * 0.05);
                },
                b"c\n" => {
                    match targeting.target {
                        Some(name) => {
                            let target = masses_clone.get(&name).unwrap();
                            let d_v = target.velocity;
                            let m_v = ship.velocity;
                            acceleration = (d_v.0 - m_v.0,
                                            d_v.1 - m_v.1,
                                            d_v.2 - m_v.2);
                        },
                        None => (),
                    }
                },
                b"t\n" => {
                    match targeting.target {
                        Some(name) => {
                            let target = masses_clone.get(&name).unwrap();
                            let d_p = target.position;
                            let m_p = ship.position;
                            acceleration = ((d_p.0 - m_p.0) * 0.01,
                                            (d_p.1 - m_p.1) * 0.01,
                                            (d_p.2 - m_p.2) * 0.01);
                        },
                        None => (),
                    }
                },
                _ => {
                    if result == 0 {
                        return false
                    }
                },
            },
            Err(_error) => (),
        }

        ship.accelerate(acceleration);

        true
    }
}
