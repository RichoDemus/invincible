use std::collections::HashMap;
use std::f64::consts::PI;
use std::ops::Not;

use itertools::Itertools;
use legion::*;
use nalgebra::{Isometry2, Point, Point2, Vector2};
use ncollide2d::query::{self, PointQuery, Proximity};
use ncollide2d::shape::Ball;
use rand::Rng;

use crate::{
    BODY_INITIAL_MASS_MAX, GRAVITATIONAL_CONSTANT, HEIGHT, INITIAL_SPEED, NUM_BODIES, SUN_SIZE,
    WIDTH,
};


pub(crate) struct Core {
    world: World,
    paused: bool,
    predicted_orbit: Option<Vec<Point2<f64>>>,
}

impl Core {
    pub(crate) fn new() -> Core {
        let world = World::default();
        Core {
            world,
            paused: false,
            predicted_orbit: None,
        }
    }

    pub(crate) fn init(&mut self) {
    }

    pub(crate) fn tick(&mut self, dt: f64, camera_x_axis: f64, camera_y_axis: f64) {
    }

    pub(crate) fn draw(&self) /* -> (Vec<Drawable>, Vec<Point2<f64>>) */ {
    }

    pub(crate) fn click(&mut self, click_position: Vector2<f64>) {
    }

    pub(crate) fn pause(&mut self) {
        self.paused = self.paused.not();
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{Isometry2, Point2, Vector2};
    use ncollide2d::query::PointQuery;

    use super::*;

    #[test]
    fn it_works() {
        let vector: Vector2<f64> = Vector2::new(11., 11.);
        let vector1 = Vector2::new(10., 10.);

        let result: Vector2<f64> = vector1 - vector;

        let result = result.magnitude();

        print!("{:?}", result)
    }

    #[test]
    fn test_click_inside() {
        let cuboid = Ball::new(1.);
        let click_pos = Point2::from(Vector2::new(11., 20.));

        let cuboid_pos = Isometry2::translation(10., 20.);

        // Solid projection.
        assert_eq!(cuboid.distance_to_point(&cuboid_pos, &click_pos, true), 0.0);
    }
}
