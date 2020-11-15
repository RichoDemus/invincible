use std::collections::HashMap;
use std::ops::Not;

use itertools::Itertools;
use nalgebra::{Isometry2, Point, Point2, Vector2};
use ncollide2d::query::PointQuery;
use ncollide2d::shape::Ball;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use uuid::Uuid;

use crate::ship::{Ship};
use crate::planet::Planet;
use crate::selectability::{Selectable, PositionAndShape, SelectableAndPositionAndShape};

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
        for _ in 0..10 {
            let id= Uuid::new_v4();
            let planet = Planet::random(id, &mut names, &mut rng);
            self.planets.insert(id, planet);
        }

        for _ in 0..1 {
            let id = Uuid::new_v4();
            self.ships.insert(id, Ship::random(id, &mut vec!["Wayfarer"], &mut rng));
        }

        // self.world.extend((0..10).map(|_| {
        //     let x = rng.gen_range(0., WIDTH as f64);
        //     let y = rng.gen_range(0., HEIGHT as f64);
        //     let name = names.pop().expect("no more planet names");
        //     (
        //         Id::default(),
        //         Name {
        //             name: String::from(name),
        //         },
        //         Planet {},
        //         Position {
        //             point: Point2::new(x, y),
        //         },
        //         Inventory {
        //             contents: vec![(
        //                 Commodity::Food,
        //                 InventoryItem {
        //                     amount: 1000,
        //                     bought_for: 0,
        //                 },
        //             )],
        //             storage_capacity: 1000,
        //         },
        //         Shape {
        //             shape: Ball::new(10.),
        //         },
        //         Selectable,
        //         Population {
        //             population: rng.gen_range(1, 10),
        //         },
        //         Market::default(),
        //     )
        // }));

        // add natural resources to some planets
        // {
        //     let mut entites = vec![];
        //     for chunk in <&mut Planet>::query().iter_chunks_mut(&mut self.world) {
        //         // println!(
        //         //     "the entities in the chunk have {:?} components",
        //         //     chunk.archetype().layout().component_types(),
        //         // );
        //         for (entity, _) in chunk.into_iter_entities() {
        //             entites.push(entity);
        //         }
        //     }
        //
        //     for entity in entites {
        //         if let Some(mut entry) = self.world.entry(entity) {
        //             // access information about the entity's archetype
        //             // println!("{:?} has {:?}", entity, entry.archetype().layout().component_types());
        //
        //             // add an extra component
        //             if rng.gen_range(0, 4) == 0 {
        //                 entry.add_component(NaturalResources {
        //                     resource: Commodity::Water,
        //                 });
        //             }
        //             // add an extra component
        //             // else if rng.gen_range(0, 3) == 0 {
        //             //     entry.add_component(NaturalResources {
        //             //         resource: resource::Hydrogen,
        //             //     });
        //             // }
        //         }
        //     }
        //
        //     // let planets = <&Planet>::query().iter(&self.world).count();
        //     // let natural_resources = <&NaturalResources>::query().iter(&self.world).count();
        //
        //     // panic!(format!("There are {} planets and {} natural resources", planets, natural_resources))
        // }

        // add ships
        // self.world.extend((0..1).map(|_| {
        //     (
        //         Id::default(),
        //         Ship {
        //             objective: Idle,
        //             max_speed: 1.,
        //         },
        //         Name {
        //             name: String::from("Wayfarer"),
        //         },
        //         Position {
        //             point: Point2::new(200., 200.),
        //         },
        //         Shape {
        //             shape: Ball::new(2.),
        //         },
        //         Selectable,
        //         Selected,
        //         Destination { destination: None },
        //         Velocity::default(),
        //         Inventory {
        //             contents: Default::default(),
        //             storage_capacity: 100,
        //         },
        //     )
        // }));
    }

    pub fn tick_day(&mut self) {
        if self.paused {
            return;
        }

        let mut sell_orders= vec![];
        let mut buy_orders= vec![];
        for planet in self.planets.values_mut() {
            planet.tick_day();
            for sell_order in &planet.sell_orders {
                sell_orders.push(sell_order);
            }
            for buy_order in &planet.buy_orders {
                buy_orders.push(buy_order);
            }
        }

        for ship in self.ships.values_mut() {
            ship.tick_day(&buy_orders, &sell_orders);
        }

        // <(&NaturalResources, &mut Inventory)>::query().for_each_mut(
        //     &mut self.world,
        //     |(natural_resources, stockpiles): (&NaturalResources, &mut Inventory)| {
        //         let produced_resource = match &natural_resources.resource {
        //             Commodity::Water => Commodity::Food,
        //             Commodity::Hydrogen => Commodity::Fuel,
        //             other => panic!(format!("Unhandled natural resource: {:?}", other)),
        //         };
        //
        //         stockpiles.add(produced_resource, 10, 0);
        //     },
        // );
        //
        // // pop eats food
        // <(&mut Inventory, &mut Population)>::query().for_each_mut(
        //     &mut self.world,
        //     |(stockpiles, population): (&mut Inventory, &mut Population)| {
        //         let current_food = stockpiles.get_amount(&Commodity::Food);
        //
        //         if current_food < 1 {
        //             //starvation
        //             population.population = population.population.saturating_sub(1);
        //         } else {
        //             stockpiles.remove(&Commodity::Food, population.population);
        //         }
        //     },
        // );
        //
        // //calculate prices
        // {
        //     <(&Inventory, &mut Market)>::query().for_each_mut(
        //         &mut self.world,
        //         |(inventory, market): (&Inventory, &mut Market)| {
        //             let food_amount = inventory.get_amount(&Commodity::Food);
        //             let food_selling_price = market_calculations::calculate_basic_selling_price(
        //                 food_amount,
        //                 inventory.storage_capacity,
        //                 0,
        //                 0,
        //             );
        //             let food_buying_price = market_calculations::calculate_basic_buying_price(
        //                 food_amount,
        //                 inventory.storage_capacity,
        //                 0,
        //                 0,
        //             );
        //             market.food_buy_price = food_buying_price;
        //             market.food_sell_price = food_selling_price;
        //         },
        //     );
        // }
        //
        // let position_lookup: HashMap<Uuid, Point2<f64>> = <(&Id, &Position)>::query()
        //     .iter(&self.world)
        //     .map(|(id, position)| (id.uuid, position.point))
        //     .collect();
        //
        // let markets: HashMap<Uuid, MarketWithPosition> = <(&Market, &Position, &Id)>::query()
        //     .iter(&self.world)
        //     .map(|(market, position, id)| {
        //         (
        //             id.uuid,
        //             MarketWithPosition {
        //                 id: id.uuid,
        //                 position: position.point,
        //                 food_buy_price: market.food_buy_price,
        //                 food_sell_price: market.food_sell_price,
        //             },
        //         )
        //     })
        //     .collect();
        //
        // // Figure out something for ships in space to do
        // {
        //     <(&mut Ship, &Position, &Inventory, &mut Destination)>::query().for_each_mut(
        //         &mut self.world,
        //         |(ship, pos, stockpiles, destination): (
        //             &mut Ship,
        //             &Position,
        //             &Inventory,
        //             &mut Destination,
        //         )| {
        //             let decision =
        //                 ship::figure_out_what_to_do_in_space(&pos.point, stockpiles, &markets);
        //             println!("Space decision: {:?}", decision);
        //             match decision {
        //                 ShipDecision::TravelTo(station) => {
        //                     destination.destination = Some((
        //                         station,
        //                         markets.get(&station).expect("asd station").position,
        //                     ));
        //                     ship.objective = ShipObjective::TravelTo(station);
        //                 }
        //                 ShipDecision::Buy(_, _) => panic!("Ships in space can't buy"),
        //                 ShipDecision::Sell(_, _) => panic!("Ships in space can't sell"),
        //             }
        //         },
        //     );
        // }
        //
        // // Figure out something for docked ships to do
        // {
        //     let mut travel_from_station_orders = vec![];
        //     let mut buy_orders = vec![];
        //     let mut sell_orders = vec![];
        //     for (uuid, stockpiles, docked) in
        //         <(&Id, &Inventory, &Docked)>::query().iter(&self.world)
        //     {
        //         for (station_id, position, _stockpiles) in
        //             <(&Id, &Position, &Inventory)>::query().iter(&self.world)
        //         {
        //             if docked.docked_at != station_id.uuid {
        //                 continue;
        //             }
        //             let ship_id = uuid.uuid;
        //             let station_id = station_id.uuid;
        //             let ship_inventory = stockpiles;
        //
        //             let ship_decision = ship::figure_out_what_to_do_at_station(
        //                 &station_id,
        //                 &position.point,
        //                 ship_inventory,
        //                 &markets,
        //             );
        //             match ship_decision {
        //                 ShipDecision::TravelTo(uuid) => {
        //                     travel_from_station_orders.push((ship_id, uuid))
        //                 }
        //                 ShipDecision::Buy(resource, amount) => {
        //                     buy_orders.push((ship_id, station_id, resource, amount))
        //                 }
        //                 ShipDecision::Sell(resource, amount) => {
        //                     sell_orders.push((ship_id, station_id, resource, amount))
        //                 }
        //             }
        //         }
        //     }
        //
        //     println!("Leaves: {:?}", travel_from_station_orders);
        //     println!("buys: {:?}", buy_orders);
        //     println!("sells: {:?}", sell_orders);
        //     // we know what ships want to do, execute it!
        //     undock_ships(
        //         travel_from_station_orders,
        //         &position_lookup,
        //         &mut self.world,
        //     );
        //
        //     //process market orders
        //     for (id, inventory) in <(&Id, &mut Inventory)>::query().iter_mut(&mut self.world) {
        //         let inventory: &mut Inventory = inventory;
        //         for (ship_id, station_id, resource, amount) in &buy_orders {
        //             let stockpile_id = id.uuid;
        //             if ship_id == &stockpile_id {
        //                 inventory.add(*resource, *amount, 0);
        //             }
        //             if station_id == &stockpile_id {
        //                 inventory.remove(resource, *amount);
        //             }
        //         }
        //         for (ship_id, station_id, resource, amount) in &sell_orders {
        //             let stockpile_id = id.uuid;
        //             if ship_id == &stockpile_id {
        //                 inventory.remove(resource, *amount);
        //             }
        //             if station_id == &stockpile_id {
        //                 inventory.add(*resource, *amount, 0);
        //             }
        //         }
        //
        //         //remove empty resources
        //         inventory.clean();
        //     }
        // }
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
                    let distance = shape.distance_to_point(
                        &Isometry2::translation(position.x, position.y),
                        &Point {
                            coords: click_position,
                        },
                        true,
                    );
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
