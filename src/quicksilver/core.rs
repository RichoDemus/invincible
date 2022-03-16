use std::collections::HashMap;
use std::ops::Not;

use itertools::Itertools;
use nalgebra::{Isometry2, Point, Point2, Vector2};
use ncollide2d::query::PointQuery;
use ncollide2d::shape::Ball;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use uuid::Uuid;
use quicksilver::log;

use crate::quicksilver::ship::{Ship, ShipDecision};
use crate::quicksilver::planet::Planet;
use crate::quicksilver::selectability::{Selectable, PositionAndShape, SelectableAndPositionAndShape};
use crate::quicksilver::market_calculations::MarketOrder;
use crate::quicksilver::projections;
use crate::quicksilver::projections::{add_id_name_mapping, id_to_name};

pub struct Core {
    pub ships: HashMap<Uuid, Ship>,
    pub planets: HashMap<Uuid, Planet>,
    paused: bool,
}

impl Core {
    pub fn new() -> Core {
        Core {
            ships: HashMap::new(),
            planets: HashMap::new(),
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

        let planets = vec![
                Planet {
                    name: "Foodsies".to_owned(),
                    water: true,
                    position: Point2::new(400., 400.),
                    population: 5,
                    ..Planet::default()
                },
                Planet {
                    name: "Biggo".to_owned(),
                    water: false,
                    position: Point2::new(600., 400.),
                    population: 5,
                    ..Planet::default()
                },
                Planet {
                    name: "Smallo".to_owned(),
                    water: false,
                    position: Point2::new(500., 600.),
                    population: 1,
                    hydrogen: true,
                    ..Planet::default()
                },
                Planet {
                    name: "Fjool".to_owned(),
                    water: false,
                    position: Point2::new(700., 600.),
                    population: 1,
                    fuel_plant:true,
                    ..Planet::default()
                },
        ];

        for planet in planets {
                add_id_name_mapping(planet.id, planet.name.clone());
                self.planets.insert(planet.id, planet);
        }

        // for _ in 0..10 {
        //     let id= Uuid::new_v4();
        //     let planet = Planet::random(id, &mut names, &mut rng);
        //     add_id_name_mapping(id, planet.name.clone());
        //     self.planets.insert(id, planet);
        // }

        for _ in 0..1 {
            let id = Uuid::new_v4();
            let ship = Ship::random(id, &mut vec!["Wayfarer"], &mut rng);
            add_id_name_mapping(id, ship.name.clone());
            self.ships.insert(id, ship);
        }

    }

    pub fn tick_day(&mut self) {
        if self.paused {
            return;
        }

        let mut transactions = vec![];
        for planet in self.planets.values_mut() {
            let mut new_transactions = planet.tick_day();
            transactions.append(&mut new_transactions);

        }

        for transaction in transactions {
            // todo do this with fancy market_actor trait?
            match (self.planets.get_mut(&transaction.seller), self.ships.get_mut(&transaction.seller)) {
                (Some(seller), None) => seller.items.remove(transaction.commodity, transaction.amount),
                (None, Some(seller)) => seller.inventory.remove(transaction.commodity, transaction.amount),
                _ => panic!("no seller for transaction: {:?}", transaction),
            };

            match (self.planets.get_mut(&transaction.buyer), self.ships.get_mut(&transaction.buyer)) {
                (Some(buyer), None) => buyer.items.add(transaction.commodity, transaction.amount),
                (None, Some(buyer)) => buyer.inventory.add(transaction.commodity, transaction.amount),
                _ => panic!("no buyer for transaction: {:?}", transaction),
            };

        }

        let market_orders = self.planets.values().flat_map(|planet|planet.market_orders.clone()).collect::<Vec<_>>();


        let mut transactions = vec![];
        for ship in self.ships.values_mut() {
            let decision = ship.tick_day(&market_orders);
            println!("Ship {} decided: {:?}", ship.name, decision);
            match decision {
                ShipDecision::Buy(buy) => {
                    log::info!("{} put buy order {:?}", ship.name, buy);
                    let mut new_transactions = self.planets.get_mut(&buy.location).expect("Should be a planet here").add_and_process_market_order(MarketOrder::BuyOrder(buy));
                    transactions.append(&mut new_transactions);
                },
                ShipDecision::Sell(sell) => {
                    println!("Put sell order {:?}", sell);
                    let mut new_transactions = self.planets.get_mut(&sell.location).expect("Should be a planet here").add_and_process_market_order(MarketOrder::SellOrder(sell));
                    transactions.append(&mut new_transactions);
                },
                ShipDecision::Nothing => {},
            }
        }

        for transaction in transactions {
            // todo do this with fancy market_actor trait?
            match (self.planets.get_mut(&transaction.seller), self.ships.get_mut(&transaction.seller)) {
                (Some(seller), None) => seller.items.remove(transaction.commodity, transaction.amount),
                (None, Some(seller)) => seller.inventory.remove(transaction.commodity, transaction.amount),
                _ => panic!("no seller for transaction: {:?}", transaction),
            };

            match (self.planets.get_mut(&transaction.buyer), self.ships.get_mut(&transaction.buyer)) {
                (Some(buyer), None) => buyer.items.add(transaction.commodity, transaction.amount),
                (None, Some(buyer)) => buyer.inventory.add(transaction.commodity, transaction.amount),
                _ => panic!("no buyer for transaction: {:?}", transaction),
            };

        }

    }

    pub fn tick(&mut self, _dt: f64, _camera_x_axis: f64, _camera_y_axis: f64) {
        if self.paused {
            return;
        }

        // todo cache this
        let position_lookup: HashMap<Uuid, Point2<f64>> = self.planets.iter()
            .map(|(id, planet)|(id.clone(), planet.position))
            .collect();

        for ship in self.ships.values_mut() {
            ship.tick(&position_lookup);
        }


        // // todo remove
        // let position_lookup: HashMap<Uuid, Point2<f64>> = <(&Id, &Position)>::query()
        //     .iter(&self.world)
        //     .map(|(id, position)| (id.uuid, position.point))
        //     .collect();
        //
        // // arrive at destination
        // {
        //     let entities_that_have_arrived = <(&Position, &Destination)>::query()
        //         .iter_chunks(&self.world)
        //         .flat_map(|chunk| chunk.into_iter_entities())
        //         .filter_map(
        //             |(entity, (position, destination)): (Entity, (&Position, &Destination))| {
        //                 if let Some((destination_id, destination_point)) = destination.destination {
        //                     Some((entity, position.point, destination_id, destination_point))
        //                 } else {
        //                     None
        //                 }
        //             },
        //         )
        //         .filter(|(_entity, position, _destination_id, destination_point)| {
        //             is_close_enough_to_dock(destination_point, position)
        //         })
        //         .collect::<Vec<_>>();
        //
        //     for (entity, position, destination_uid, _desitination_point) in
        //         entities_that_have_arrived
        //     {
        //         if let Some(mut entry) = self.world.entry(entity) {
        //             entry.add_component(Docked {
        //                 docked_at: destination_uid,
        //             });
        //             let destination = entry
        //                 .get_component_mut::<Destination>()
        //                 .expect("Should have a destination");
        //             destination.destination = None;
        //             entry.remove_component::<Velocity>();
        //             entry.remove_component::<Position>();
        //             println!(
        //                 "Docked with {:?}({:?}) at: {:?}",
        //                 destination_uid,
        //                 position_lookup.get(&destination_uid),
        //                 position
        //             );
        //         }
        //     }
        // }
        //
        // // move entities with a destination towards it
        // <(&mut Position, &mut Velocity, &Destination)>::query().for_each_mut(
        //     &mut self.world,
        //     |(position, velocity, destination): (&mut Position, &mut Velocity, &Destination)| {
        //         if let Some((_, destination)) = destination.destination {
        //             let vector: Vector2<f64> = destination - position.point;
        //             let vector = vector.normalize(); //maybe not needed here
        //
        //             let new_velocity = velocity.velocity + vector;
        //             let new_velocity = new_velocity.normalize();
        //             velocity.velocity = new_velocity;
        //
        //             //todo move to separate thing:
        //             position.point += new_velocity;
        //         }
        //     },
        // );
    }

    pub fn click(&mut self, click_position: Vector2<f64>) {
        const MINIMUM_CLICK_DISTANCE_TO_EVEN_CONSIDER: f64 = 5f64;

        let planets = self.planets.values_mut().map(|planet|{
            // let selectable: &mut dyn Selectable + &mut dyn PositionAndShape = planet;
            let selectable: &mut dyn SelectableAndPositionAndShape = planet;
            selectable
        });
        let ships = self.ships.values_mut().map(|ship|{
            let selectable: &mut dyn SelectableAndPositionAndShape = ship;
            selectable
        });

        let selected_thing = planets.chain(ships)
            .map(|planet|{
                //hacky, deselect everything now
                planet.deselect();
                    let (position, shape) = planet.position_and_shape();
                    // let distance = shape.distance_to_point(
                    //     &ncollide2d::Isometry2::translation(position.x, position.y),
                    //     &Point {
                    //         coords: click_position,
                    //     },
                    //     true,
                    // )
                let distance= 0.;
                    (planet, distance)
            })
            .filter(|(_, distance) | distance < &MINIMUM_CLICK_DISTANCE_TO_EVEN_CONSIDER)
            .fold1(|(left_planet, left_distance), (right_planet, right_distance)| {
                if left_distance < right_distance {
                    return (left_planet, left_distance)
                } else {
                    (right_planet, right_distance)
                }
            });

        if let Some((selectable, _)) = selected_thing {
            selectable.select();
        }
    }

    pub fn pause(&mut self) {
        self.paused = self.paused.not();
    }
}

// fn undock_ships(
//     ships_that_are_leaving: Vec<(Uuid, Uuid)>,
//     position_lookup: &HashMap<Uuid, Point2<f64>>,
//     world: &mut World,
// ) {
//     let ship_ids = ships_that_are_leaving
//         .iter()
//         .map(|(id, _)| id)
//         .collect::<Vec<_>>();
//
//     let entities_that_are_leaving: Vec<(Entity, Id, Docked)> = <(&Id, &Docked)>::query()
//         .filter(component::<Ship>())
//         .iter_chunks(world)
//         .flat_map(|chunk| chunk.into_iter_entities())
//         .filter(|(_, (id, _))| ship_ids.contains(&&id.uuid))
//         .map(|(entity, (id, docked))| (entity, *id, *docked))
//         .collect::<Vec<_>>();
//
//     for (entity, id, docked_at) in entities_that_are_leaving {
//         if let Some(mut entry) = world.entry(entity) {
//             let station_position = *position_lookup
//                 .get(&docked_at.docked_at)
//                 .expect("Should be a station here");
//
//             entry.add_component(Position {
//                 point: station_position,
//             });
//             entry.add_component(Velocity {
//                 velocity: Vector2::new(0., 0.),
//             });
//
//             let (_, move_to) = ships_that_are_leaving
//                 .iter()
//                 .find(|(ship, _)| ship == &id.uuid)
//                 .expect("should totes be a station here");
//             let move_to_position = position_lookup
//                 .get(move_to)
//                 .expect("should be a thing here");
//             {
//                 let destination = entry
//                     .get_component_mut::<Destination>()
//                     .expect("Should have a destination");
//                 destination.destination = Some((*move_to, *move_to_position));
//             }
//             {
//                 let ship = entry
//                     .get_component_mut::<Ship>()
//                     .expect("Should have a ship");
//                 ship.objective = ShipObjective::TravelTo(*move_to);
//             }
//             entry.remove_component::<Docked>();
//             println!(
//                 "Undocked from {:?} at: {:?}",
//                 docked_at.docked_at, station_position,
//             );
//         }
//     }
// }

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
