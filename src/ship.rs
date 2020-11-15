use std::collections::HashMap;

use nalgebra::{Point2, Vector2};
use ncollide2d::shape::Ball;
use rand::prelude::StdRng;
use rand::Rng;
use uuid::Uuid;

use crate::{HEIGHT, WIDTH, market_calculations};
use crate::selectability::{Selectable, PositionAndShape, SelectableAndPositionAndShape};
use crate::market_calculations::{Commodity, BuyOrder, SellOrder};
use crate::inventory::Inventory;
use crate::planet::Planet;

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
        let x = rng.gen_range(0., WIDTH as f64);
        let y = rng.gen_range(0., HEIGHT as f64);
        let name = names.pop().expect("no more planet names");
        Ship {
            id,
            name: String::from(name),
            position: Point2::new(x, y),
            velocity: Vector2::new(0.,0.),
            docked_at: None,
            shape: Ball::new(2.),
            selected: false,
            objective: ShipObjective::Idle,
            inventory: Inventory::with_capacity(100),
        }
    }

    pub fn tick_day(&mut self, buy_orders: &Vec<&BuyOrder>, sell_orders: &Vec<&SellOrder>) -> ShipDecision {
        if self.inventory.space_left() < 1 {
            let destination =
                market_calculations::calculate_where_to_sell_cargo(&self.position, self.inventory.get(&Commodity::Food), buy_orders);
            let destination = destination.expect("No where to go");
            self.objective = ShipObjective::TravelTo(destination);
        } else {
            // cargo empty, lets buy
            let destination = market_calculations::calculate_where_to_buy_frakking_food(&self.position, sell_orders);
            let destination = destination.expect("No where to go");

            if self.docked_at.is_none() {
                self.objective = ShipObjective::TravelTo(destination);
                self.docked_at = None;
            } else if let Some(docked_station) = self.docked_at {
                //we're docked were we want to be
                let local_sell_orders = sell_orders.iter()
                    .filter(|sell_order|sell_order.location == docked_station)
                    .collect::<Vec<_>>();
                let buy_order = market_calculations::create_buy_order(self.inventory.space_left(), Commodity::Food, self.id, local_sell_orders);
                return ShipDecision::Buy(buy_order);
            } else {
                // docked at wrong station
                self.objective = ShipObjective::TravelTo(destination)
            }
        }
        ShipDecision::Nothing
    }

    pub fn tick(&mut self, position_lookup: &HashMap<Uuid, Point2<f64>>) {
        match self.objective {
            ShipObjective::TravelTo(destination_id) => {
                    let destination = position_lookup.get(&destination_id).expect("destination should exist");
                            let vector: Vector2<f64> = destination - self.position;
                            if vector.magnitude() < 5. {
                                //close enough to dock
                                self.docked_at = Some(destination_id);
                                self.objective = ShipObjective::Idle;
                            } else {
                            let vector = vector.normalize(); //maybe not needed here

                            let new_velocity = self.velocity + vector;
                            let new_velocity = new_velocity.normalize();
                self.velocity = new_velocity;

                            //todo move to separate thing:
                            self.position += new_velocity;

                            }

                // check if there, then dock

            }
            ShipObjective::Idle => {}
        }
    }

}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShipObjective {
    Idle,
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
