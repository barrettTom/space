use crate::constants;
use crate::mass::Mass;
use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tractorbeam {
    pub status: TractorbeamStatus,
    strength: f64,
    desired_distance: Option<f64>,
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

impl Tractorbeam {
    pub fn new() -> Tractorbeam {
        Tractorbeam {
            status: TractorbeamStatus::None,
            strength: constants::SHIP_TRACTORBEAM_STRENGTH,
            desired_distance: None,
        }
    }

    pub fn toggle_pull(&mut self) {
        self.status = match self.status {
            TractorbeamStatus::None => TractorbeamStatus::Pull,
            TractorbeamStatus::Push => TractorbeamStatus::Pull,
            TractorbeamStatus::Bring => TractorbeamStatus::Pull,
            TractorbeamStatus::Pull => TractorbeamStatus::None,
        }
    }

    pub fn toggle_push(&mut self) {
        self.status = match self.status {
            TractorbeamStatus::None => TractorbeamStatus::Push,
            TractorbeamStatus::Pull => TractorbeamStatus::Push,
            TractorbeamStatus::Bring => TractorbeamStatus::Push,
            TractorbeamStatus::Push => TractorbeamStatus::None,
        }
    }

    pub fn toggle_bring(&mut self, desired_distance: f64) {
        self.desired_distance = Some(desired_distance);
        self.status = match self.status {
            TractorbeamStatus::None => TractorbeamStatus::Bring,
            TractorbeamStatus::Pull => TractorbeamStatus::Bring,
            TractorbeamStatus::Push => TractorbeamStatus::Bring,
            TractorbeamStatus::Bring => TractorbeamStatus::None,
        }
    }

    pub fn off(&mut self) {
        self.status = TractorbeamStatus::None;
    }

    pub fn get_acceleration(&self, position: Vector, target: Mass) -> Vector {
        let acceleration = position.clone() - target.position.clone();
        match self.status {
            TractorbeamStatus::Push => acceleration.unitize() * -0.05,
            TractorbeamStatus::Pull => acceleration.unitize() * 0.05,
            TractorbeamStatus::Bring => match self.desired_distance {
                Some(desired_distance) => {
                    if desired_distance > position.distance_from(target.position) {
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
}
