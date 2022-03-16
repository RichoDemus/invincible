use std::collections::HashMap;

use nalgebra::{Point2, Vector2};
use ncollide2d::shape::Ball;
use quicksilver::log;
use rand::prelude::StdRng;
use rand::Rng;
use uuid::Uuid;

use crate::quicksilver::inventory::Inventory;
use crate::quicksilver::market_calculations::{BuyOrder, Commodity, MarketOrder, SellOrder};
use crate::quicksilver::planet::Planet;
use crate::quicksilver::projections::id_to_name;
use crate::quicksilver::selectability::{
    PositionAndShape, Selectable, SelectableAndPositionAndShape,
};
use crate::quicksilver::{market_calculations, HEIGHT, WIDTH};

pub struct Ship {
    pub id: Uuid,
    pub name: String,
    pub position: Point2<f64>,
    pub velocity: Vector2<f64>,
    pub docked_at: Option<Uuid>,
    pub shape: Ball<f64>,
    pub selected: bool,
    pub objective: ShipObjective,
    pub inventory: Inventory,
}

impl Ship {
    pub fn random(id: Uuid, names: &mut Vec<&str>, rng: &mut StdRng) -> Self {
        let x = rng.gen_range(0.0..WIDTH as f64);
        let y = rng.gen_range(0.0..HEIGHT as f64);
        let name = names.pop().expect("no more planet names");
        Ship {
            id,
            name: String::from(name),
            position: Point2::new(x, y),
            velocity: Vector2::new(0., 0.),
            docked_at: None,
            shape: Ball::new(2.),
            selected: false,
            objective: ShipObjective::Idle("new".to_string()),
            inventory: Inventory::with_capacity(100),
        }
    }

    pub fn tick_day(&mut self, market_orders: &Vec<MarketOrder>) -> ShipDecision {
        log::info!(
            "{}: {:?}. dock: {:?}",
            self.name,
            self.objective,
            self.docked_at.map(|id| id_to_name(&id).value().clone())
        );
        if self.inventory.space_left() < 1 {
            let destination = market_calculations::calculate_where_to_sell_cargo(
                &self.position,
                self.inventory.get(&Commodity::Food),
                market_orders,
            );
            if destination.is_none() {
                self.objective = ShipObjective::Idle("nowhere to sell".to_string());
                log::info!("Nowhere to sell");
                return ShipDecision::Nothing;
            }
            let destination = destination.expect("No where to go");

            if self.docked_at.is_none() {
                self.objective = ShipObjective::TravelTo(destination);
            } else {
                if self.docked_at.unwrap() == destination {
                    //we're docked were we want to be
                    let local_sell_orders = market_orders
                        .iter()
                        .filter_map(|order| match order {
                            MarketOrder::SellOrder(_) => None,
                            MarketOrder::BuyOrder(order) => Some(order),
                        })
                        .filter(|buy_order| buy_order.location == destination)
                        .collect::<Vec<_>>();
                    // todo consider current sell orders before placing new
                    let sell_order = market_calculations::create_sell_order(
                        self.inventory.get(&Commodity::Food),
                        Commodity::Food,
                        self.id,
                        local_sell_orders,
                        destination,
                    );
                    log::info!("placed sell order: {:?}", sell_order);
                    return ShipDecision::Sell(sell_order);
                }

                self.objective = ShipObjective::TravelTo(destination);
                self.docked_at = None;
            }
            log::info!(
                "{} travel to planet {} to sell",
                self.name,
                id_to_name(&destination).value()
            );
            return ShipDecision::Nothing;
        } else {
            // cargo empty, lets buy
            let destination = market_calculations::calculate_where_to_buy_frakking_food(
                &self.position,
                market_orders,
            );
            if destination.is_none() {
                log::info!("Couldn't find anything to buy");
                self.objective = ShipObjective::Idle("nowhere to buy".to_string());
                return ShipDecision::Nothing;
            }
            let destination = destination.expect("No where to go");

            if self.docked_at.is_none() {
                self.objective = ShipObjective::TravelTo(destination);
                self.docked_at = None;
                log::info!("travel to {} to buy", id_to_name(&destination).value());
                return ShipDecision::Nothing;
            }

            let docked_station = self.docked_at.unwrap();

            if docked_station == destination {
                //we're docked were we want to be
                let local_sell_orders = market_orders
                    .iter()
                    .filter_map(|order| match order {
                        MarketOrder::BuyOrder(_) => None,
                        MarketOrder::SellOrder(order) => Some(order),
                    })
                    .filter(|sell_order| sell_order.location == docked_station)
                    .collect::<Vec<_>>();
                // todo consider current buy orders before placing new
                let buy_order = market_calculations::create_buy_order(
                    self.inventory.space_left(),
                    Commodity::Food,
                    self.id,
                    local_sell_orders,
                    docked_station,
                );
                log::info!("placed buy order: {:?}", buy_order);
                return ShipDecision::Buy(buy_order);
            } else {
                // docked at wrong station
                self.objective = ShipObjective::TravelTo(destination);
                log::info!("docked at wrong station");
                return ShipDecision::Nothing;
            }
        }
    }

    pub fn tick(&mut self, position_lookup: &HashMap<Uuid, Point2<f64>>) {
        match self.objective {
            ShipObjective::TravelTo(destination_id) => {
                let destination = position_lookup
                    .get(&destination_id)
                    .expect("destination should exist");
                let vector: Vector2<f64> = destination - self.position;
                if vector.magnitude() < 5. {
                    //close enough to dock
                    self.docked_at = Some(destination_id);
                    self.objective = ShipObjective::Idle("docked".to_string());
                    log::info!("Docked at {}", id_to_name(&destination_id).value());
                } else {
                    let vector = vector.normalize(); //maybe not needed here

                    let new_velocity = self.velocity + vector;
                    let new_velocity = new_velocity.normalize();
                    self.velocity = new_velocity * 2.;

                    //todo move to separate thing:
                    self.position += new_velocity;
                }

                // check if there, then dock
            }
            ShipObjective::Idle(_) => {}
        }
    }
}

#[derive(Debug)]
pub enum ShipDecision {
    Buy(BuyOrder),
    Sell(SellOrder),
    Nothing,
}

impl SelectableAndPositionAndShape for Ship {}

impl Selectable for Ship {
    fn selected(&self) -> bool {
        self.selected
    }

    fn select(&mut self) {
        self.selected = true;
    }

    fn deselect(&mut self) {
        self.selected = false;
    }
}

impl PositionAndShape for Ship {
    fn position_and_shape(&self) -> (Point2<f64>, Ball<f64>) {
        (self.position, self.shape)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShipObjective {
    Idle(String),
    TravelTo(Uuid),
    //DockedAt(Uuid),
}

//
// #[derive(Debug)]
// pub enum ShipDecision {
//     TravelTo(Uuid),
//     Buy(Commodity, u64),
//     Sell(Commodity, u64),
// }

// pub fn figure_out_what_to_do_in_space(
//     position: &Point2<f64>,
//     ship_inventory: &Inventory,
//     markets: &HashMap<Uuid, MarketWithPosition>,
// ) -> ShipDecision {
//     if ship_inventory.space_left() < 1 {
//         //cargo full, lets go sell
//         let markets = markets.values().cloned().collect::<Vec<_>>();
//         let inventory = ship_inventory
//             .contents
//             .iter()
//             .map(|(res, item)| (*res, *item))
//             .collect::<Vec<_>>();
//         let destination =
//             market_calculations::calculate_where_to_sell_cargo(position, &inventory, markets);
//         let destination = destination.expect("No where to go");
//         ShipDecision::TravelTo(destination)
//     } else {
//         // cargo empty, lets buy
//         let markets = markets.values().cloned().collect::<Vec<_>>();
//         let destination =
//             market_calculations::calculate_where_to_buy_frakking_food(position, markets);
//         let destination = destination.expect("No where to go");
//         ShipDecision::TravelTo(destination)
//     }
// }

// pub fn figure_out_what_to_do_at_station(
//     station_id: &Uuid,
//     station_position: &Point2<f64>,
//     ship_inventory: &Inventory,
//     markets: &HashMap<Uuid, MarketWithPosition>,
// ) -> ShipDecision {
//     let markets = markets.values().cloned().collect::<Vec<_>>();
//
//     // cargo full, sell here or go elsewhere?
//     if ship_inventory.space_left() < 1 {
//         let inventory = ship_inventory
//             .contents
//             .iter()
//             .map(|(res, amount)| (*res, *amount))
//             .collect::<Vec<_>>();
//         let destination = market_calculations::calculate_where_to_sell_cargo(
//             station_position,
//             &inventory,
//             markets,
//         );
//         let destination = destination.expect("No where to go");
//         if &destination != station_id {
//             // we wanna go elsewhere to sell
//             return ShipDecision::TravelTo(destination);
//         }
//
//         // sell here
//         return ShipDecision::Sell(Commodity::Food, 100);
//     }
//
//     // should we go somewhere else to buy?
//     let destination =
//         market_calculations::calculate_where_to_buy_frakking_food(station_position, markets);
//     let destination = destination.expect("No where to go");
//
//     if &destination != station_id {
//         return ShipDecision::TravelTo(destination);
//     }
//
//     //buy here!
//     ShipDecision::Buy(Commodity::Food, 100)
// }
