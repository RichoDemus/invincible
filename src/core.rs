use std::ops::Not;

use itertools::Itertools;
use legion::*;
use nalgebra::{Isometry2, Point, Point2, Vector2};
use ncollide2d::query::PointQuery;
use ncollide2d::shape::Ball;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use crate::components::{
    Name, NaturalResources, Planet, Position, Resource, Selectable, Selected, Shape, Stockpiles,
};
use crate::{HEIGHT, WIDTH};

pub(crate) struct Core {
    pub(crate) world: World,
    paused: bool,
}

impl Core {
    pub(crate) fn new() -> Core {
        let world = World::default();
        Core {
            world,
            paused: false,
        }
    }

    pub(crate) fn init(&mut self) {
        let mut names = vec![
            "denkenia",
            "chilliter",
            "gillomia",
            "uccoth",
            "sohines",
            "keizuno",
            "chialia",
            "laumia",
            "venethea",
            "zaniophus",
            "zubrorth",
            "uthora",
            "chinus",
            "chonus",
            "buabos",
            "lavuter",
            "lacreatera",
            "kemacury",
            "kubbides",
            "gendion",
            "unus",
            "thuinope",
            "davuter",
            "varoruta",
        ];

        let mut rng = StdRng::seed_from_u64(0);
        self.world.extend((0..10).map(|_| {
            let x = rng.gen_range(0., WIDTH as f64);
            let y = rng.gen_range(0., HEIGHT as f64);
            let name = names.pop().expect("no more planet names");
            (
                Name {
                    name: String::from(name),
                },
                Planet {},
                Position {
                    point: Point2::new(x, y),
                },
                Stockpiles::default(),
                Shape {
                    shape: Ball::new(10.),
                },
                Selectable,
            )
        }));

        // add natural resources to some planets
        {
            let mut entites = vec![];
            for chunk in <&mut Planet>::query().iter_chunks_mut(&mut self.world) {
                // println!(
                //     "the entities in the chunk have {:?} components",
                //     chunk.archetype().layout().component_types(),
                // );
                for (entity, _) in chunk.into_iter_entities() {
                    entites.push(entity);
                }
            }

            for entity in entites {
                if let Some(mut entry) = self.world.entry(entity) {
                    // access information about the entity's archetype
                    // println!("{:?} has {:?}", entity, entry.archetype().layout().component_types());

                    // add an extra component
                    if rng.gen_range(0, 3) == 0 {
                        entry.add_component(NaturalResources {
                            resource: Resource::Water,
                        });
                    }
                }
            }

            // let planets = <&Planet>::query().iter(&self.world).count();
            // let natural_resources = <&NaturalResources>::query().iter(&self.world).count();

            // panic!(format!("There are {} planets and {} natural resources", planets, natural_resources))
        }
    }

    pub(crate) fn tick(&mut self, _dt: f64, _camera_x_axis: f64, _camera_y_axis: f64) {
        <(&NaturalResources, &mut Stockpiles)>::query().for_each_mut(
            &mut self.world,
            |(natural_resources, stockpiles): (&NaturalResources, &mut Stockpiles)| {
                let has_resource = stockpiles
                    .stockpiles
                    .iter()
                    .any(|(resource, _)| resource == &natural_resources.resource);
                if !has_resource {
                    stockpiles
                        .stockpiles
                        .push((natural_resources.resource.clone(), 0));
                }

                stockpiles.stockpiles = stockpiles
                    .stockpiles
                    .clone()
                    .into_iter()
                    .map(|(resource, amount)| {
                        if resource == natural_resources.resource {
                            (resource, amount + 1)
                        } else {
                            (resource, amount)
                        }
                    })
                    .collect();
            },
        );
    }

    pub(crate) fn click(&mut self, click_position: Vector2<f64>) {
        let mut query = <(&Position, &Shape)>::query().filter(component::<Selectable>());
        let clicked_entity = query
            .iter_chunks(&self.world)
            .flat_map(|chunk| chunk.into_iter_entities())
            .map(
                |(entity, (position, shape)): (Entity, (&Position, &Shape))| {
                    let distance = shape.shape.distance_to_point(
                        &Isometry2::translation(position.point.x, position.point.y),
                        &Point {
                            coords: click_position,
                        },
                        true,
                    );
                    (entity, distance)
                },
            )
            .filter(|(_, distance)| distance < &5f64)
            .sorted_by(|(_, left_distance), (_, right_distance)| {
                left_distance
                    .partial_cmp(right_distance)
                    .expect("couldn't unwrap ordering")
            })
            .next()
            .map(|(entity, _)| entity);

        let selected_entities = <&Selectable>::query()
            .filter(component::<Selected>())
            .iter_chunks(&self.world)
            .flat_map(|chunk| chunk.into_iter_entities())
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

        for entity in selected_entities {
            if let Some(mut entry) = self.world.entry(entity) {
                entry.remove_component::<Selected>();
            }
        }

        if let Some(entity) = clicked_entity {
            // clicked something
            if let Some(mut entry) = self.world.entry(entity) {
                entry.add_component(Selected);
            }
        }
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
