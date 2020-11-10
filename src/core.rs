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
use uuid::Uuid;

use crate::components::Resource::Food;
use crate::components::{
    Id, Name, NaturalResources, Planet, Population, Position, Resource, Selectable, Selected,
    Shape, Stockpiles,
};
use crate::economy_components::Market;
use crate::market_calculations::MarketWithPosition;
use crate::ship::ShipDecision;
use crate::ship_components::ShipObjective::Idle;
use crate::ship_components::{
    is_close_enough_to_dock, Destination, Docked, Ship, ShipObjective, Velocity,
};
use crate::{market_calculations, ship, HEIGHT, WIDTH};

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
                    // else if rng.gen_range(0, 3) == 0 {
                    //     entry.add_component(NaturalResources {
                    //         resource: Resource::Hydrogen,
                    //     });
                    // }
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
                Selected,
                Destination { destination: None },
                Velocity::default(),
                Stockpiles {
                    stockpiles: Default::default(),
                    size: 100,
                },
            )
        }));
    }

    pub fn tick_day(&mut self) {
        if self.paused {
            return;
        }

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

        let position_lookup: HashMap<Uuid, Point2<f64>> = <(&Id, &Position)>::query()
            .iter(&self.world)
            .map(|(id, position)| (id.uuid, position.point))
            .collect();

        let markets: HashMap<Uuid, MarketWithPosition> = <(&Market, &Position, &Id)>::query()
            .iter(&self.world)
            .map(|(market, position, id)| {
                (
                    id.uuid,
                    MarketWithPosition {
                        id: id.uuid,
                        position: position.point,
                        food_buy_price: market.food_buy_price,
                        food_sell_price: market.food_sell_price,
                    },
                )
            })
            .collect();

        // // Figure out something for idle ships to do
        // {
        //     let markets = <(&Market, &Position, &Id)>::query()
        //         .iter(&self.world)
        //         .map(|(market, position, id)| MarketWithPosition {
        //             id: id.uuid,
        //             position: position.point,
        //             food_buy_price: market.food_buy_price,
        //             food_sell_price: market.food_sell_price,
        //         })
        //         .collect::<Vec<_>>();
        //
        //     <(&mut Ship, &Position, &mut Destination)>::query().for_each_mut(
        //         &mut self.world,
        //         |(ship, pos, destination): (&mut Ship, &Position, &mut Destination)| {
        //             if ship.objective == Idle {
        //                 let most_profitable_route =
        //                     market_calculations::get_most_profitable_route(&markets, &pos.point);
        //                 ship.objective = ShipObjective::TravelTo(most_profitable_route.source.0);
        //                 destination.destination = Some((
        //                     most_profitable_route.source.0,
        //                     most_profitable_route.source.1,
        //                 ));
        //             }
        //         },
        //     );
        // }
        //
        // // Handle docked ships, buy and sell or just move again
        // {
        //     let docked_ships: Vec<(Uuid, Docked, Stockpiles)> =
        //         <(&Id, &Docked, &Stockpiles)>::query()
        //             .filter(component::<Ship>())
        //             .iter(&self.world)
        //             .map(|(id, docked, stockpiles)| (id.uuid, *docked, stockpiles.clone()))
        //             .collect::<Vec<_>>();
        //
        //     let mut buy_orders = vec![];
        //     let mut move_orders = HashMap::new();
        //     for (id, docked_at, _stockpiles) in docked_ships {
        //         let current_position = markets
        //             .get(&docked_at.docked_at)
        //             .expect("the ship should be at a station")
        //             .position;
        //         let markets_with_positions = markets.values().cloned().collect::<Vec<_>>();
        //         let most_profitable_route = market_calculations::get_most_profitable_route(
        //             &markets_with_positions,
        //             &current_position,
        //         );
        //         println!("Most profitable route: {:?}", most_profitable_route);
        //         if most_profitable_route.source.0 != docked_at.docked_at {
        //             println!("\t wrong station, moving");
        //             move_orders.insert(id, most_profitable_route.source.0);
        //         } else {
        //             println!("Creating buy order");
        //             let ship_id = id;
        //             let station_id = docked_at.docked_at;
        //             let commodity = most_profitable_route.commodity;
        //             let amount = 100; // todo calculate this
        //             buy_orders.push((ship_id, station_id, commodity, amount));
        //         }
        //     }
        //
        //     let ids_of_ship_that_are_leaving =
        //         move_orders.iter().map(|(id, _)| id).collect::<Vec<_>>();
        //     let entities_that_are_leaving: Vec<(Entity, Id, Docked)> = <(&Id, &Docked)>::query()
        //         .filter(component::<Ship>())
        //         .iter_chunks(&self.world)
        //         .flat_map(|chunk| chunk.into_iter_entities())
        //         .filter(|(_entity, (id, _docked_at))| {
        //             ids_of_ship_that_are_leaving.contains(&&id.uuid)
        //         })
        //         .map(|(entity, (id, docked_at))| (entity, *id, *docked_at))
        //         .collect::<Vec<_>>();
        //
        //     for (entity, id, docked_at) in entities_that_are_leaving {
        //         if let Some(mut entry) = self.world.entry(entity) {
        //             entry.add_component(Position {
        //                 point: markets
        //                     .get(&docked_at.docked_at)
        //                     .expect("Should be a station here")
        //                     .position,
        //             });
        //             entry.add_component(Velocity {
        //                 velocity: Vector2::new(0., 0.),
        //             });
        //
        //             let move_to = move_orders
        //                 .get(&id.uuid)
        //                 .expect("should totes be a station here");
        //             let move_to_pos = markets
        //                 .get(&move_to)
        //                 .expect("this is getting tiring")
        //                 .position;
        //
        //             {
        //                 let destination = entry
        //                     .get_component_mut::<Destination>()
        //                     .expect("Should have a destination");
        //                 destination.destination = Some((*move_to, move_to_pos));
        //             }
        //             {
        //                 let ship = entry
        //                     .get_component_mut::<Ship>()
        //                     .expect("Should have a ship");
        //                 ship.objective = ShipObjective::TravelTo(*move_to);
        //             }
        //             entry.remove_component::<Docked>();
        //         }
        //     }
        //
        //     <(&Id, &mut Stockpiles)>::query()
        //         .filter(component::<Planet>())
        //         .for_each_mut(
        //             &mut self.world,
        //             |(id, stockpiles): (&Id, &mut Stockpiles)| {
        //                 let id = id.uuid;
        //                 if let Some((_ship_id, _market_id, commodity, amount)) = buy_orders
        //                     .iter()
        //                     .find(|(_, market_id, _, _)| market_id == &id)
        //                 {
        //                     *stockpiles
        //                         .stockpiles
        //                         .get_mut(commodity)
        //                         .expect("this station should have this stockpile") -= amount;
        //                 }
        //             },
        //         );
        //
        //     <(&Id, &mut Stockpiles)>::query()
        //         .filter(component::<Ship>())
        //         .for_each_mut(
        //             &mut self.world,
        //             |(id, stockpiles): (&Id, &mut Stockpiles)| {
        //                 let id = id.uuid;
        //                 if let Some((_ship_id, _market_id, commodity, amount)) =
        //                     buy_orders.iter().find(|(ship_id, _, _, _)| ship_id == &id)
        //                 {
        //                     *stockpiles.stockpiles.entry(*commodity).or_insert(0) += amount;
        //                 }
        //             },
        //         );
        //
        // }

        // Figure out something for ships in space to do
        {
            <(&mut Ship, &Position, &Stockpiles, &mut Destination)>::query().for_each_mut(
                &mut self.world,
                |(_ship, pos, stockpiles, destination): (
                    &mut Ship,
                    &Position,
                    &Stockpiles,
                    &mut Destination,
                )| {
                    let decision =
                        ship::figure_out_what_to_do_in_space(&pos.point, stockpiles, &markets);
                    println!("Space decision: {:?}", decision);
                    match decision {
                        ShipDecision::TravelTo(station) => {
                            destination.destination = Some((
                                station,
                                markets.get(&station).expect("asd station").position,
                            ))
                        }
                        ShipDecision::Buy(_, _) => panic!("Ships in space can't buy"),
                        ShipDecision::Sell(_, _) => panic!("Ships in space can't sell"),
                    }
                },
            );
        }

        // Figure out something for docked ships to do
        {
            let mut travel_from_station_orders = vec![];
            let mut buy_orders = vec![];
            let mut sell_orders = vec![];
            for (uuid, stockpiles, docked) in
                <(&Id, &Stockpiles, &Docked)>::query().iter(&self.world)
            {
                for (station_id, _stockpiles) in <(&Id, &Stockpiles)>::query().iter(&self.world) {
                    if docked.docked_at != station_id.uuid {
                        continue;
                    }
                    let ship_id = uuid.uuid;
                    let station_id = station_id.uuid;
                    let ship_inventory = stockpiles;

                    let ship_decision = ship::figure_out_what_to_do_at_station(
                        &station_id,
                        ship_inventory,
                        &markets,
                    );
                    match ship_decision {
                        ShipDecision::TravelTo(uuid) => {
                            travel_from_station_orders.push((ship_id, uuid))
                        }
                        ShipDecision::Buy(resource, amount) => {
                            buy_orders.push((ship_id, station_id, resource, amount))
                        }
                        ShipDecision::Sell(resource, amount) => {
                            sell_orders.push((ship_id, station_id, resource, amount))
                        }
                    }
                }
            }

            println!("Leaves: {:?}", travel_from_station_orders);
            println!("buys: {:?}", buy_orders);
            println!("sells: {:?}", sell_orders);
            // we know what ships want to do, execute it!
            undock_ships(
                travel_from_station_orders,
                &position_lookup,
                &mut self.world,
            );

            //process market orders
            for (id, stockpiles) in <(&Id, &mut Stockpiles)>::query().iter_mut(&mut self.world) {
                let stockpiles: &mut Stockpiles = stockpiles;
                for (ship_id, station_id, resource, amount) in &buy_orders {
                    let stockpile_id = id.uuid;
                    if ship_id == &stockpile_id {
                        *stockpiles.stockpiles.entry(*resource).or_insert(0) += amount;
                    }
                    if station_id == &stockpile_id {
                        *stockpiles.stockpiles.entry(*resource).or_insert(0) -= amount;
                    }
                }
                for (ship_id, station_id, resource, amount) in &sell_orders {
                    let stockpile_id = id.uuid;
                    if ship_id == &stockpile_id {
                        *stockpiles.stockpiles.entry(*resource).or_insert(0) -= amount;
                    }
                    if station_id == &stockpile_id {
                        *stockpiles.stockpiles.entry(*resource).or_insert(0) += amount;
                    }
                }

                //remove empty resources
                stockpiles.stockpiles = stockpiles
                    .stockpiles
                    .iter()
                    .filter(|(_key, value)| **value > 0)
                    .map(|(key, value)| (*key, *value))
                    .collect();
            }
        }
    }

    pub fn tick(&mut self, _dt: f64, _camera_x_axis: f64, _camera_y_axis: f64) {
        if self.paused {
            return;
        }

        // todo remove
        let position_lookup: HashMap<Uuid, Point2<f64>> = <(&Id, &Position)>::query()
            .iter(&self.world)
            .map(|(id, position)| (id.uuid, position.point))
            .collect();

        // arrive at destination
        {
            let entities_that_have_arrived = <(&Position, &Destination)>::query()
                .iter_chunks(&self.world)
                .flat_map(|chunk| chunk.into_iter_entities())
                .filter_map(
                    |(entity, (position, destination)): (Entity, (&Position, &Destination))| {
                        if let Some((destination_id, destination_point)) = destination.destination {
                            Some((entity, position.point, destination_id, destination_point))
                        } else {
                            None
                        }
                    },
                )
                .filter(|(_entity, position, _destination_id, destination_point)| {
                    is_close_enough_to_dock(destination_point, position)
                })
                .collect::<Vec<_>>();

            for (entity, position, destination_uid, _desitination_point) in
                entities_that_have_arrived
            {
                if let Some(mut entry) = self.world.entry(entity) {
                    entry.add_component(Docked {
                        docked_at: destination_uid,
                    });
                    let destination = entry
                        .get_component_mut::<Destination>()
                        .expect("Should have a destination");
                    destination.destination = None;
                    entry.remove_component::<Velocity>();
                    entry.remove_component::<Position>();
                    println!(
                        "Docked with {:?}({:?}) at: {:?}",
                        destination_uid,
                        position_lookup.get(&destination_uid),
                        position
                    );
                }
            }
        }

        // move entities with a destination towards it
        <(&mut Position, &mut Velocity, &Destination)>::query().for_each_mut(
            &mut self.world,
            |(position, velocity, destination): (&mut Position, &mut Velocity, &Destination)| {
                if let Some((_, destination)) = destination.destination {
                    // println!("printerino");
                    let vector: Vector2<f64> = destination - position.point;
                    // println!("\t{:?} - {:?} = {:?}", vector, destination, position.point);
                    let vector = vector.normalize(); //maybe not needed here
                                                     // println!("\tnormalized: {:?}", vector);

                    let new_velocity = velocity.velocity + vector;
                    // println!("\tnew velocity: {:?} = {:?} + {:?}", new_velocity, velocity.velocity, vector);
                    let new_velocity = new_velocity.normalize();
                    // println!("\tnew velocity: {:?}", velocity);
                    // println!("\tship point: {:?}, velocity: {:?} heading to {:?}", position.point, velocity.velocity, destination);
                    velocity.velocity = new_velocity;

                    //todo move to separate thing:
                    position.point += new_velocity;
                    // println!("\tship point: {:?}, velocity: {:?} heading to {:?}", position.point, velocity.velocity, destination);
                    // println!();
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

fn undock_ships(
    ships_that_are_leaving: Vec<(Uuid, Uuid)>,
    position_lookup: &HashMap<Uuid, Point2<f64>>,
    world: &mut World,
) {
    let ship_ids = ships_that_are_leaving
        .iter()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    let entities_that_are_leaving: Vec<(Entity, Id, Docked)> = <(&Id, &Docked)>::query()
        .filter(component::<Ship>())
        .iter_chunks(world)
        .flat_map(|chunk| chunk.into_iter_entities())
        .filter(|(_, (id, _))| ship_ids.contains(&&id.uuid))
        .map(|(entity, (id, docked))| (entity, *id, *docked))
        .collect::<Vec<_>>();

    for (entity, id, docked_at) in entities_that_are_leaving {
        if let Some(mut entry) = world.entry(entity) {
            let station_position = *position_lookup
                .get(&docked_at.docked_at)
                .expect("Should be a station here");

            entry.add_component(Position {
                point: station_position,
            });
            entry.add_component(Velocity {
                velocity: Vector2::new(0., 0.),
            });

            let (_, move_to) = ships_that_are_leaving
                .iter()
                .find(|(ship, _)| ship == &id.uuid)
                .expect("should totes be a station here");
            let move_to_position = position_lookup
                .get(move_to)
                .expect("should be a thing here");
            {
                let destination = entry
                    .get_component_mut::<Destination>()
                    .expect("Should have a destination");
                destination.destination = Some((*move_to, *move_to_position));
            }
            {
                let ship = entry
                    .get_component_mut::<Ship>()
                    .expect("Should have a ship");
                ship.objective = ShipObjective::TravelTo(*move_to);
            }
            entry.remove_component::<Docked>();
            println!(
                "Undocked from {:?} at: {:?}",
                docked_at.docked_at, station_position,
            );
            // println!("pos component: {:?}", entry.get_component::<Position>());
            // panic!(format!("Undocked at: {:?}", station_position));
        }
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
