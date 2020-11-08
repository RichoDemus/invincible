use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Not;

use itertools::Itertools;
use legion::*;
use nalgebra::{Isometry2, Point, Point2, Vector2};
use ncollide2d::query::PointQuery;
use ncollide2d::shape::Ball;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use crate::components::Resource::Food;
use crate::components::{
    Id, Name, NaturalResources, Planet, Population, Position, Resource, Selectable, Selected,
    Shape, Stockpiles,
};
use crate::economy_components::Market;
use crate::market_calculations::MarketWithPosition;
use crate::ship_components::ShipObjective::Idle;
use crate::ship_components::{Destination, Ship, ShipObjective, Velocity};
use crate::{market_calculations, HEIGHT, WIDTH};

pub struct Core {
    pub world: World,
    paused: bool,
}

impl Core {
    pub fn new() -> Core {
        let world = World::default();
        Core {
            world,
            paused: false,
        }
    }

    pub fn init(&mut self) {
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
            let mut stockpiles = HashMap::new();
            stockpiles.insert(Food, 1000);
            (
                Id::default(),
                Name {
                    name: String::from(name),
                },
                Planet {},
                Position {
                    point: Point2::new(x, y),
                },
                Stockpiles {
                    stockpiles,
                    size: 1000,
                },
                Shape {
                    shape: Ball::new(10.),
                },
                Selectable,
                Population {
                    population: rng.gen_range(1, 10),
                },
                Market::default(),
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
                    if rng.gen_range(0, 4) == 0 {
                        entry.add_component(NaturalResources {
                            resource: Resource::Water,
                        });
                    }
                    // add an extra component
                    else if rng.gen_range(0, 3) == 0 {
                        entry.add_component(NaturalResources {
                            resource: Resource::Hydrogen,
                        });
                    }
                }
            }

            // let planets = <&Planet>::query().iter(&self.world).count();
            // let natural_resources = <&NaturalResources>::query().iter(&self.world).count();

            // panic!(format!("There are {} planets and {} natural resources", planets, natural_resources))
        }

        // add ships
        self.world.extend((0..1).map(|_| {
            (
                Id::default(),
                Ship {
                    objective: Idle,
                    max_speed: 1.,
                },
                Name {
                    name: String::from("Wayfarer"),
                },
                Position {
                    point: Point2::new(200., 200.),
                },
                Shape {
                    shape: Ball::new(2.),
                },
                Selectable,
                Destination { destination: None },
                Velocity::default(),
            )
        }));
    }

    pub fn tick_day(&mut self) {
        <(&NaturalResources, &mut Stockpiles)>::query().for_each_mut(
            &mut self.world,
            |(natural_resources, stockpiles): (&NaturalResources, &mut Stockpiles)| {
                let produced_resource = match &natural_resources.resource {
                    Resource::Water => Resource::Food,
                    Resource::Hydrogen => Resource::Fuel,
                    other => panic!(format!("Unhandled natural resource: {:?}", other)),
                };

                let current_stockpile =
                    *stockpiles.stockpiles.get(&produced_resource).unwrap_or(&0);
                let new_stockpile = min(stockpiles.size, current_stockpile + 10);
                stockpiles
                    .stockpiles
                    .insert(produced_resource, new_stockpile);
            },
        );

        <(&mut Stockpiles, &mut Population)>::query().for_each_mut(
            &mut self.world,
            |(stockpiles, population): (&mut Stockpiles, &mut Population)| {
                let current_food = *stockpiles.stockpiles.get(&Food).unwrap_or(&0);
                if current_food < 1 {
                    //starvation
                    population.population = population.population.saturating_sub(1);
                } else {
                    let new_food = max(0, current_food.saturating_sub(population.population));
                    *stockpiles
                        .stockpiles
                        .get_mut(&Food)
                        .expect("There should be food here") = new_food;
                }
            },
        );

        //calculate prices
        {
            <(&Stockpiles, &mut Market)>::query().for_each_mut(
                &mut self.world,
                |(stockpiles, market): (&Stockpiles, &mut Market)| {
                    let food_amount = *stockpiles.stockpiles.get(&Food).unwrap_or(&0);
                    let food_selling_price = market_calculations::calculate_basic_selling_price(
                        food_amount,
                        stockpiles.size,
                        0,
                        0,
                    );
                    let food_buying_price = market_calculations::calculate_basic_buying_price(
                        food_amount,
                        stockpiles.size,
                        0,
                        0,
                    );
                    market.food_buy_price = food_buying_price;
                    market.food_sell_price = food_selling_price;
                },
            );
        }

        // Figure out something for idle ships to do
        {
            let markets = <(&Market, &Position, &Id)>::query()
                .iter(&self.world)
                .map(|(market, position, id)| MarketWithPosition {
                    id: id.uuid,
                    position: position.point,
                    food_buy_price: market.food_buy_price,
                    food_sell_price: market.food_sell_price,
                })
                .collect::<Vec<_>>();

            <(&mut Ship, &Position, &mut Destination)>::query().for_each_mut(
                &mut self.world,
                |(ship, pos, destination): (&mut Ship, &Position, &mut Destination)| {
                    if ship.objective == Idle {
                        let most_profitable_route =
                            market_calculations::get_most_profitable_route(&markets, &pos.point);
                        ship.objective = ShipObjective::TravelTo(most_profitable_route.source.0);
                        destination.destination = Some((
                            most_profitable_route.source.0,
                            most_profitable_route.source.1,
                        ));
                    }
                },
            );
        }
    }

    pub fn tick(&mut self, _dt: f64, _camera_x_axis: f64, _camera_y_axis: f64) {
        <(&mut Position, &mut Velocity, &Destination)>::query().for_each_mut(
            &mut self.world,
            |(position, velocity, destination): (&mut Position, &mut Velocity, &Destination)| {
                if let Some((_, destination)) = destination.destination {
                    let vector: Vector2<f64> = destination - position.point;
                    let vector = vector.normalize(); //maybe not needed here

                    let new_velocity = velocity.velocity + vector;
                    let new_velocity = new_velocity.normalize();
                    velocity.velocity = new_velocity;

                    // panic!("Vector and new: : {:?}, {:?}", vector, new_velocity);

                    //todo move to separate thing:
                    position.point += new_velocity;
                }
            },
        );
    }

    pub fn click(&mut self, click_position: Vector2<f64>) {
        const MINIMUM_CLICK_DISTANCE_TO_EVEN_CONSIDER: f64 = 5f64;
        // find clicked entity
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
            .filter(|(_, distance)| distance < &MINIMUM_CLICK_DISTANCE_TO_EVEN_CONSIDER)
            .sorted_by(|(_, left_distance), (_, right_distance)| {
                left_distance
                    .partial_cmp(right_distance)
                    .expect("couldn't unwrap ordering")
            })
            .next()
            .map(|(entity, _)| entity);

        // deselect everything
        {
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
        }

        // Select clicked entity
        if let Some(entity) = clicked_entity {
            // clicked something
            if let Some(mut entry) = self.world.entry(entity) {
                entry.add_component(Selected);
            }
        }
    }

    pub fn pause(&mut self) {
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
