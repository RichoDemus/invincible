use std::collections::HashMap;

use nalgebra::Point2;
use ncollide2d::shape::Ball;
use rand::prelude::StdRng;
use rand::Rng;
use uuid::Uuid;

use crate::{HEIGHT, WIDTH};
use crate::selectability::{Selectable, PositionAndShape, SelectableAndPositionAndShape};
use crate::market_calculations::Commodity;
use crate::inventory::Inventory;

pub struct Ship {
    pub id: Uuid,
    pub name: String,
    pub position: Point2<f64>,
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
            shape: Ball::new(2.),
            selected: false,
            objective: ShipObjective::Idle,
            inventory: Inventory::with_capacity(100),
        }
    }
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
