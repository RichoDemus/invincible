use nalgebra::Point2;
use ncollide2d::shape::Ball;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Id {
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
pub(crate) struct Position {
    pub(crate) point: Point2<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Planet {}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Name {
    pub(crate) name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Resource {
    Water,
    Food,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NaturalResources {
    pub resource: Resource,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Stockpiles {
    pub stockpiles: HashMap<Resource, u64>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct Selected;

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct Selectable;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Shape {
    pub shape: Ball<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Population {
    pub population: u64,
}
