use crate::constants;
use crate::mass::Mass;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EnginesStatus {
    None,
    ApproachingTargetVelocity,
}

impl Default for EnginesStatus {
    fn default() -> Self {
        EnginesStatus::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Engines {
    acceleration: Vector,
    target_velocity: Option<Vector>,
    pub fuel: f64,
}

impl Engines {
    pub fn new() -> Engines {
        Engines {
            acceleration: Vector::default(),
            target_velocity: None,
            fuel: constants::SHIP_ENGINES_FUEL_START,
        }
    }

    pub fn give_recv(
        &mut self,
        recv: String,
        position: Vector,
        velocity: Vector,
        target: Option<&Mass>,
    ) {
        let mut acceleration = Vector::default();
        match recv.as_str() {
            "5" => acceleration.x += 0.1,
            "0" => acceleration.x -= 0.1,
            "8" => acceleration.y += 0.1,
            "2" => acceleration.y -= 0.1,
            "4" => acceleration.z += 0.1,
            "6" => acceleration.z -= 0.1,
            "+" => acceleration = velocity * 0.05,
            "-" => {
                acceleration = velocity * -1.05;
            }
            "s" => {
                acceleration = velocity * -1.0;
            }
            "c" => {
                if let Some(target) = target {
                    acceleration = target.velocity.clone() - velocity;
                }
            }
            "t" => {
                if let Some(target) = target {
                    acceleration = (target.position.clone() - position) * 0.01;
                }
            }
            _ => (),
        }
        self.acceleration = acceleration;
    }

    pub fn take_acceleration(&mut self) -> Vector {
        let acceleration = self.acceleration.clone();
        self.acceleration = Vector::default();

        if self.fuel - acceleration.magnitude() >= 0.0 {
            self.fuel -= acceleration.magnitude();
            acceleration
        } else {
            Vector::default()
        }
    }
}
