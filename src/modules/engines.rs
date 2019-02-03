use crate::mass::Mass;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Engines {
    acceleration: (f64, f64, f64),
}

impl Engines {
    pub fn new() -> Engines {
        Engines {
            acceleration: (0.0, 0.0, 0.0),
        }
    }

    pub fn recv_acceleration(&mut self) -> (f64, f64, f64) {
        let acceleration = self.acceleration;
        self.acceleration = (0.0, 0.0, 0.0);
        acceleration
    }

    pub fn give_client_data(&mut self, ship: &Mass, target: Option<&Mass>, data: String) {
        let mut acceleration = (0.0, 0.0, 0.0);
        match data.as_bytes() {
            b"5\n" => acceleration.0 += 0.1,
            b"0\n" => acceleration.0 -= 0.1,
            b"8\n" => acceleration.1 += 0.1,
            b"2\n" => acceleration.1 -= 0.1,
            b"4\n" => acceleration.2 += 0.1,
            b"6\n" => acceleration.2 -= 0.1,
            b"+\n" => {
                let m_v = ship.velocity;
                acceleration = (m_v.0 * 0.05, m_v.1 * 0.05, m_v.2 * 0.05);
            }
            b"-\n" => {
                let m_v = ship.velocity;
                acceleration = (
                    -1.0 * m_v.0 * 0.05,
                    -1.0 * m_v.1 * 0.05,
                    -1.0 * m_v.2 * 0.05,
                );
            }
            b"s\n" => {
                let m_v = ship.velocity;
                acceleration = (-1.0 * m_v.0, -1.0 * m_v.1, -1.0 * m_v.2);
            }
            b"c\n" => {
                if let Some(target) = target {
                    let d_v = target.velocity;
                    let m_v = ship.velocity;
                    acceleration = (d_v.0 - m_v.0, d_v.1 - m_v.1, d_v.2 - m_v.2);
                }
            }
            b"t\n" => {
                if let Some(target) = target {
                    let d_p = target.position;
                    let m_p = ship.position;
                    acceleration = (
                        (d_p.0 - m_p.0) * 0.01,
                        (d_p.1 - m_p.1) * 0.01,
                        (d_p.2 - m_p.2) * 0.01,
                    );
                }
            }
            _ => (),
        }
        self.acceleration = acceleration;
    }
}
