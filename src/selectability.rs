use ncollide2d::shape::Ball;
use nalgebra::Point2;

pub trait SelectableAndPositionAndShape: Selectable + PositionAndShape {}

pub trait Selectable {
    fn selected(&self) -> bool;
    fn select(&mut self);
    fn deselect(&mut self);
}

pub trait PositionAndShape {
    fn position_and_shape(&self) -> (Point2<f64>, Ball<f64>);
}
