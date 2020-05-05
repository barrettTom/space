use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Dashboard {}

impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard {}
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClientDashboard {
    position: Vector,
}
