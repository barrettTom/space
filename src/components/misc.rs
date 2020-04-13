use crate::math::Vector;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Acceleration(pub Vector);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Velocity(pub Vector);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position(pub Vector);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Name(pub String);
