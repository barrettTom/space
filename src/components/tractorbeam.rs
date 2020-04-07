use crate::components::types::ModuleType;
use crate::constants;
use crate::mass::{Mass, MassType};
use crate::math::ControlSystem;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tractorbeam {
    pub range: f64,
    pub status: Status,
    strength: f64,
    desired_distance: Option<f64>,
    control_system: ControlSystem,
}

impl Tractorbeam {
    pub fn new() -> Tractorbeam {
        Tractorbeam {
            range: constants::SHIP_TRACTORBEAM_RANGE,
            status: Status::None,
            strength: constants::SHIP_TRACTORBEAM_STRENGTH,
            desired_distance: None,
            control_system: ControlSystem::new(ModuleType::Tractorbeam),
        }
    }

    pub fn process(&mut self, ship_position: Vector, target: &mut Mass) -> bool {
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
                Status::Acquire => {
                    match target.mass_type {
                        MassType::Item { .. } => (),
                        _ => {
                            self.status = Status::None;
                            Vector::default();
                        }
                    }
                    if distance > constants::SHIP_TRACTORBEAM_ACQUIRE_RANGE {
                        direction.unitize()
                            * self.control_system.compute(
                                self.strength,
                                distance,
                                constants::SHIP_TRACTORBEAM_ACQUIRE_RANGE,
                            )
                    } else {
                        self.status = Status::None;
                        return true;
                    }
                }
                Status::None => Vector::default(),
            };

            target.effects.give_acceleration(acceleration);
        }
        false
    }

    pub fn get_client_data(&self, target: Option<&Mass>) -> String {
        let client_data = ClientData {
            desired_distance: self.desired_distance,
            has_target: target.is_some(),
            status: self.status.clone(),
        };

        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    pub fn give_received_data(&mut self, recv: String) {
        let server_recv_data: Result<ServerRecvData, serde_json::Error> =
            serde_json::from_str(&recv);
        if let Ok(server_recv_data) = server_recv_data {
            match server_recv_data.key.as_ref() {
                "o" => self.toggle_pull(),
                "p" => self.toggle_push(),
                "b" => self.toggle_bring(server_recv_data.desired_distance),
                "a" => self.toggle_acquire(),
                _ => (),
            }
        }
    }

    fn toggle_pull(&mut self) {
        self.status = match self.status {
            Status::Pull => Status::None,
            _ => Status::Pull,
        }
    }

    fn toggle_push(&mut self) {
        self.status = match self.status {
            Status::Push => Status::None,
            _ => Status::Push,
        }
    }

    fn toggle_bring(&mut self, desired_distance: Option<f64>) {
        self.desired_distance = desired_distance;
        self.control_system = ControlSystem::new(ModuleType::Tractorbeam);
        self.status = match self.status {
            Status::Bring => Status::None,
            _ => Status::Bring,
        }
    }

    fn toggle_acquire(&mut self) {
        self.control_system = ControlSystem::new(ModuleType::Tractorbeam);
        self.status = match self.status {
            Status::Acquire => Status::None,
            _ => Status::Acquire,
        }
    }

    fn off(&mut self) {
        self.status = Status::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub has_target: bool,
    pub desired_distance: Option<f64>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerRecvData {
    pub key: String,
    pub desired_distance: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    None,
    Push,
    Pull,
    Bring,
    Acquire,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}
