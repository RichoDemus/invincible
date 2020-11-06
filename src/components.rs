use std::collections::HashMap;

use nalgebra::Point2;
use ncollide2d::shape::Ball;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Id {
    pub uuid: Uuid,
}

impl Default for Id {
    fn default() -> Self {
        Id {
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub point: Point2<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Planet {}

#[derive(Clone, Debug, PartialEq)]
pub struct Name {
    pub name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Resource {
    Water,
    Food,
    Hydrogen,
    Fuel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NaturalResources {
    pub resource: Resource,
}

#[derive(Clone, Debug)]
pub struct Stockpiles {
    pub stockpiles: HashMap<Resource, u64>,
    pub size: u64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Selected;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Selectable;

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub shape: Ball<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Population {
    pub population: u64,
}
