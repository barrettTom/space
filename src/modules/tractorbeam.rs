use crate::constants;
use crate::mass::Mass;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tractorbeam {
    pub range: f64,
    pub status: Status,
    strength: f64,
    desired_distance: Option<f64>,
    control_system: ControlSystem,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ControlSystem {
    previous_error: f64,
    integral: f64,
    kp: f64,
    ki: f64,
    kd: f64,
    dt: f64,
}

impl ControlSystem {
    pub fn new() -> ControlSystem {
        ControlSystem {
            previous_error: 0.0,
            integral: 0.0,
            kp: 1.0,
            ki: 0.01,
            kd: 0.001,
            dt: 0.0001,
        }
    }

    pub fn compute(&mut self, strength: f64, distance: f64, desired_distance: f64) -> f64 {
        let error = desired_distance - distance;
        self.integral += error * self.dt;
        let derivative = (error - self.previous_error) / self.dt;
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.previous_error = error;

        if output.abs() > strength {
            if output.is_sign_positive() {
                strength
            } else {
                strength * -1.0
            }
        } else {
            output
        }
    }
}

impl Tractorbeam {
    pub fn new() -> Tractorbeam {
        Tractorbeam {
            range: constants::SHIP_TRACTORBEAM_RANGE,
            status: Status::None,
            strength: constants::SHIP_TRACTORBEAM_STRENGTH,
            desired_distance: None,
            control_system: ControlSystem::new(),
        }
    }

    pub fn process(&mut self, ship_position: Vector, target: &mut Mass) {
        let distance = ship_position.distance_from(target.position.clone());
        if self.range < distance {
            self.off()
        } else {
            let direction = target.position.clone() - ship_position.clone();
            let acceleration = match self.status {
                Status::Push => direction.unitize() * self.strength,
                Status::Pull => direction.unitize() * -1.0 * self.strength,
                Status::Bring => match self.desired_distance {
                    Some(desired_distance) => {
                        direction.unitize()
                            * self
                                .control_system
                                .compute(self.strength, distance, desired_distance)
                    }
                    None => Vector::default(),
                },
                Status::None => Vector::default(),
            };

            target.effects.give_acceleration(acceleration);
        }
    }

    pub fn get_client_data(&self, target: Option<&Mass>) -> String {
        let client_data = ClientData {
            has_target: target.is_some(),
            status: self.status.clone(),
        };

        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    pub fn give_received_data(&mut self, recv: String) {
        match recv.as_str() {
            "o" => self.toggle_pull(),
            "p" => self.toggle_push(),
            "b" => self.toggle_bring(constants::SHIP_TRACTORBEAM_BRING_TO_DISTANCE),
            _ => (),
        }
    }

    fn toggle_pull(&mut self) {
        self.status = match self.status {
            Status::None => Status::Pull,
            Status::Push => Status::Pull,
            Status::Bring => Status::Pull,
            Status::Pull => Status::None,
        }
    }

    fn toggle_push(&mut self) {
        self.status = match self.status {
            Status::None => Status::Push,
            Status::Pull => Status::Push,
            Status::Bring => Status::Push,
            Status::Push => Status::None,
        }
    }

    fn toggle_bring(&mut self, desired_distance: f64) {
        self.desired_distance = Some(desired_distance);
        self.control_system = ControlSystem::new();
        self.status = match self.status {
            Status::None => Status::Bring,
            Status::Pull => Status::Bring,
            Status::Push => Status::Bring,
            Status::Bring => Status::None,
        }
    }

    fn off(&mut self) {
        self.status = Status::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub has_target: bool,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    None,
    Push,
    Pull,
    Bring,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}
