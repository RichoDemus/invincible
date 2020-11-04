use nalgebra::Point2;
use ncollide2d::shape::Ball;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Resource {
    Water,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NaturalResources {
    pub resource: Resource,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct Stockpiles {
    pub stockpiles: Vec<(Resource, u64)>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct Selected;

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct Selectable;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Shape {
    pub shape: Ball<f64>,
}
