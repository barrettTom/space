use crate::math::Vector;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClientDashboard {
    position: Vector,
}
