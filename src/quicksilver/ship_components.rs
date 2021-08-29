use nalgebra::{Point2, Vector2};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShipObjective {
    Idle,
    TravelTo(Uuid),
    //DockedAt(Uuid),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ship {
    pub objective: ShipObjective,
    pub max_speed: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Velocity {
    pub velocity: Vector2<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Destination {
    pub destination: Option<(Uuid, Point2<f64>)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Docked {
    pub(crate) docked_at: Uuid,
}

pub fn is_close_enough_to_dock(left: &Point2<f64>, right: &Point2<f64>) -> bool {
    let distance: Vector2<f64> = left - right;
    distance.magnitude() < 3.
}
