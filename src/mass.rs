pub trait Mass {
    fn new(name : &str, location : (isize, isize, isize)) -> Self where Self: Sized;
    fn get_name(&self) -> &String;
    fn get_location(&self) -> (isize, isize, isize);
    fn give_location(&mut self, location : (isize, isize, isize));
    fn serialize(&self) -> String;
}
