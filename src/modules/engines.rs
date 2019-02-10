use crate::mass::Mass;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Engines {
    acceleration: Vector,
}

impl Engines {
    pub fn new() -> Engines {
        Engines {
            acceleration: Vector::default(),
        }
    }

    pub fn recv_acceleration(&mut self) -> Vector {
        let acceleration = self.acceleration.clone();
        self.acceleration = Vector::default();
        acceleration
    }

    pub fn give_client_data(&mut self, ship: &Mass, target: Option<&Mass>, data: String) {
        let mut acceleration = Vector::default();
        match data.as_bytes() {
            b"5\n" => acceleration.a += 0.1,
            b"0\n" => acceleration.a -= 0.1,
            b"8\n" => acceleration.b += 0.1,
            b"2\n" => acceleration.b -= 0.1,
            b"4\n" => acceleration.c += 0.1,
            b"6\n" => acceleration.c -= 0.1,
            b"+\n" => acceleration = ship.velocity.clone() * 0.05,
            b"-\n" => {
                acceleration = ship.velocity.clone() * -1.05;
            }
            b"s\n" => {
                acceleration = ship.velocity.clone() * -1.0;
            }
            b"c\n" => {
                if let Some(target) = target {
                    acceleration = target.velocity.clone() - ship.velocity.clone();
                }
            }
            b"t\n" => {
                if let Some(target) = target {
                    acceleration = (target.position.clone() - ship.position.clone()) * 0.01;
                }
            }
            _ => (),
        }
        self.acceleration = acceleration;
    }
}
