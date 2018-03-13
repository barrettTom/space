use downcast::Any;

pub trait Mass : Any {
    fn name(&self) -> &String;
    fn position(&self) -> (f64, f64, f64);
    fn serialize(&self) -> String;
    fn process(&mut self);
    fn give_acceleration(&mut self, acceleration : (f64, f64, f64));
    fn recv_velocity(&self) -> (f64, f64, f64);
    fn box_clone(&self) -> Box<Mass>;
}

impl Clone for Box<Mass> {
    fn clone(&self) -> Box<Mass> {
        self.box_clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    Ship,
    Astroid,
}

downcast!(Mass);
