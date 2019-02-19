use crate::constants;
use crate::mass::Mass;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tractorbeam {
    pub status: TractorbeamStatus,
    strength: f64,
    desired_distance: Option<f64>,
}

impl Tractorbeam {
    pub fn new() -> Tractorbeam {
        Tractorbeam {
            status: TractorbeamStatus::None,
            strength: constants::SHIP_TRACTORBEAM_STRENGTH,
            desired_distance: None,
        }
    }

    pub fn process(&mut self) {}

    pub fn get_client_data(&self, target: Option<&Mass>) -> String {
        let client_data = TractorbeamClientData {
            has_target: target.is_some(),
            status: self.status.clone(),
        };

        serde_json::to_string(&client_data).unwrap() + "\n"
    }

    pub fn give_received_data(&mut self, recv: String) {
        match recv.as_str() {
            "o" => self.toggle_pull(),
            "p" => self.toggle_push(),
            "b" => self.toggle_bring(5.0),
            _ => (),
        }
    }

    pub fn get_acceleration(&self, ship_position: Vector, target_position: Vector) -> Vector {
        let acceleration = ship_position.clone() - target_position.clone();
        match self.status {
            TractorbeamStatus::Push => acceleration.unitize() * -0.05,
            TractorbeamStatus::Pull => acceleration.unitize() * 0.05,
            TractorbeamStatus::Bring => match self.desired_distance {
                Some(desired_distance) => {
                    if desired_distance > ship_position.distance_from(target_position) {
                        acceleration.unitize() * -0.05
                    //some sort of velocity limiter
                    //if target.speed_torwards(ship) < 10.0 {
                    //    acceleration.unitize() * -0.05
                    //} else {
                    //    Vector::default()
                    //}
                    } else {
                        acceleration.unitize() * 0.05
                    }
                }
                None => Vector::default(),
            },
            TractorbeamStatus::None => Vector::default(),
        }
    }

    fn toggle_pull(&mut self) {
        self.status = match self.status {
            TractorbeamStatus::None => TractorbeamStatus::Pull,
            TractorbeamStatus::Push => TractorbeamStatus::Pull,
            TractorbeamStatus::Bring => TractorbeamStatus::Pull,
            TractorbeamStatus::Pull => TractorbeamStatus::None,
        }
    }

    fn toggle_push(&mut self) {
        self.status = match self.status {
            TractorbeamStatus::None => TractorbeamStatus::Push,
            TractorbeamStatus::Pull => TractorbeamStatus::Push,
            TractorbeamStatus::Bring => TractorbeamStatus::Push,
            TractorbeamStatus::Push => TractorbeamStatus::None,
        }
    }

    fn toggle_bring(&mut self, desired_distance: f64) {
        self.desired_distance = Some(desired_distance);
        self.status = match self.status {
            TractorbeamStatus::None => TractorbeamStatus::Bring,
            TractorbeamStatus::Pull => TractorbeamStatus::Bring,
            TractorbeamStatus::Push => TractorbeamStatus::Bring,
            TractorbeamStatus::Bring => TractorbeamStatus::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TractorbeamClientData {
    pub has_target: bool,
    pub status: TractorbeamStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TractorbeamStatus {
    None,
    Push,
    Pull,
    Bring,
}

impl Default for TractorbeamStatus {
    fn default() -> Self {
        TractorbeamStatus::None
    }
}
