use downcast::Any;

pub trait Mass : Any {
    fn new(name : &str, location : (isize, isize, isize)) -> Self where Self: Sized;
    fn name(&self) -> &String;
    fn location(&self) -> (isize, isize, isize);
    fn set_location(&mut self, location : (isize, isize, isize));
    fn serialize(&self) -> String;
}

downcast!(Mass);
