use crate::constants;
use crate::mass::Mass;
use crate::math::Vector;
use crate::modules::navigation::NavigationStatus;

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

    pub fn process(&mut self, velocity: Vector) {
        if let Some(target_velocity) = self.target_velocity.clone() {
            self.acceleration += target_velocity - velocity;
            if self.acceleration == Vector::default() {
                self.target_velocity = None
            }
        }
    }

    pub fn give_received_data(
        &mut self,
        recv: String,
        position: Vector,
        velocity: Vector,
        target: Option<&Mass>,
    ) {
        let mut acceleration = Vector::default();
        match recv.as_str() {
            "5" => acceleration.x += constants::SHIP_ENGINES_ACCELERATION,
            "0" => acceleration.x -= constants::SHIP_ENGINES_ACCELERATION,
            "8" => acceleration.y += constants::SHIP_ENGINES_ACCELERATION,
            "2" => acceleration.y -= constants::SHIP_ENGINES_ACCELERATION,
            "4" => acceleration.z += constants::SHIP_ENGINES_ACCELERATION,
            "6" => acceleration.z -= constants::SHIP_ENGINES_ACCELERATION,
            "+" => acceleration = velocity.unitize() * constants::SHIP_ENGINES_ACCELERATION,
            "-" => acceleration = velocity.unitize() * -1.0 * constants::SHIP_ENGINES_ACCELERATION,
            "s" => self.target_velocity = Some(Vector::default()),
            "c" => {
                if let Some(target) = target {
                    self.target_velocity = Some(target.velocity.clone());
                }
            }
            "t" => {
                if let Some(target) = target {
                    acceleration = (target.position.clone() - position).unitize()
                        * constants::SHIP_ENGINES_ACCELERATION;
                }
            }
            _ => (),
        }
        self.acceleration = acceleration;
    }

    pub fn get_client_data(&self, status: NavigationStatus) -> String {
        let client_data = EnginesClientData {
            has_target: status == NavigationStatus::Targeted,
            fuel: self.fuel,
        };
        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    pub fn take_acceleration(&mut self) -> Vector {
        let mut acceleration = self.acceleration.clone();
        self.acceleration = Vector::default();

        if acceleration.magnitude() >= constants::SHIP_ENGINES_ACCELERATION {
            acceleration = acceleration.unitize() * constants::SHIP_ENGINES_ACCELERATION;
        }
        if self.fuel - acceleration.magnitude() >= 0.0 {
            self.fuel -= acceleration.magnitude();
            acceleration
        } else {
            Vector::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnginesClientData {
    pub has_target: bool,
    pub fuel: f64,
}
