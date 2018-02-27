use downcast::Any;

pub trait Mass : Any {
    fn name(&self) -> &String;
    fn position(&self) -> (f64, f64, f64);
    fn serialize(&self) -> String;
    fn process(&mut self);
    fn slow(&mut self);
    fn give_acceleration(&mut self, acceleration : (f64, f64, f64));
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Ship,
    Astroid,
}

downcast!(Mass);
