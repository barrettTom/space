use crate::constants;
use crate::mass::Mass;
use crate::math::Vector;
use crate::modules::navigation;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Engines {
    pub status: Status,
    acceleration: Vector,
    target_velocity: Option<Vector>,
    pub fuel: f64,
}

impl Engines {
    pub fn new() -> Engines {
        Engines {
            status: Status::None,
            acceleration: Vector::default(),
            target_velocity: None,
            fuel: constants::SHIP_ENGINES_FUEL_START,
        }
    }

    pub fn process(&mut self, position: Vector, velocity: Vector, target: Option<&Mass>) {
        if self.target_velocity.is_none() && self.status != Status::None {
            if self.status == Status::Stopping {
                self.target_velocity = Some(Vector::default());
            }
            if let Some(target) = target {
                match self.status {
                    Status::TowardsTarget => {
                        self.acceleration = (target.position.clone() - position).unitize()
                            * constants::SHIP_ENGINES_ACCELERATION;
                        self.status = Status::None;
                    }
                    Status::FollowingTarget => self.target_velocity = Some(target.velocity.clone()),
                    _ => (),
                }
            } else {
                self.status = Status::None;
            }
        }

        if let Some(target_velocity) = self.target_velocity.clone() {
            self.acceleration += target_velocity - velocity;
            if self.acceleration == Vector::default() {
                self.target_velocity = None;
                self.status = Status::None;
            }
        }
    }

    pub fn give_received_data(&mut self, recv: String, velocity: Vector) {
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
            "s" => self.status = Status::Stopping,
            "c" => self.status = Status::FollowingTarget,
            "t" => self.status = Status::TowardsTarget,
            _ => (),
        }
        self.acceleration = acceleration;
    }

    pub fn get_client_data(&self, status: navigation::Status) -> String {
        let client_data = ClientData {
            has_target: status == navigation::Status::Targeted,
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
pub struct ClientData {
    pub has_target: bool,
    pub fuel: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    None,
    Stopping,
    FollowingTarget,
    TowardsTarget,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}
