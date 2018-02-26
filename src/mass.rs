use downcast::Any;

pub trait Mass : Any {
    fn new(name : &str, location : (f64, f64, f64)) -> Self where Self: Sized;
    fn name(&self) -> &String;
    fn location(&self) -> (f64, f64, f64);
    fn set_location(&mut self, location : (f64, f64, f64));
    fn serialize(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Ship,
    Astroid,
}

downcast!(Mass);
