use std::collections::HashMap;

use nalgebra::Point2;
use uuid::Uuid;

use crate::components::{Commodity, Inventory};
use crate::market_calculations;
use crate::market_calculations::MarketWithPosition;

#[derive(Debug)]
pub enum ShipDecision {
    TravelTo(Uuid),
    Buy(Commodity, u64),
    Sell(Commodity, u64),
}

pub fn figure_out_what_to_do_in_space(
    position: &Point2<f64>,
    ship_inventory: &Inventory,
    markets: &HashMap<Uuid, MarketWithPosition>,
) -> ShipDecision {
    if ship_inventory.space_left() < 1 {
        //cargo full, lets go sell
        let markets = markets.values().cloned().collect::<Vec<_>>();
        let inventory = ship_inventory
            .contents
            .iter()
            .map(|(res, item)| (*res, *item))
            .collect::<Vec<_>>();
        let destination =
            market_calculations::calculate_where_to_sell_cargo(position, &inventory, markets);
        let destination = destination.expect("No where to go");
        ShipDecision::TravelTo(destination)
    } else {
        // cargo empty, lets buy
        let markets = markets.values().cloned().collect::<Vec<_>>();
        let destination =
            market_calculations::calculate_where_to_buy_frakking_food(position, markets);
        let destination = destination.expect("No where to go");
        ShipDecision::TravelTo(destination)
    }
}

pub fn figure_out_what_to_do_at_station(
    station_id: &Uuid,
    station_position: &Point2<f64>,
    ship_inventory: &Inventory,
    markets: &HashMap<Uuid, MarketWithPosition>,
) -> ShipDecision {
    let markets = markets.values().cloned().collect::<Vec<_>>();

    // cargo full, sell here or go elsewhere?
    if ship_inventory.space_left() < 1 {
        let inventory = ship_inventory
            .contents
            .iter()
            .map(|(res, amount)| (*res, *amount))
            .collect::<Vec<_>>();
        let destination = market_calculations::calculate_where_to_sell_cargo(
            station_position,
            &inventory,
            markets,
        );
        let destination = destination.expect("No where to go");
        if &destination != station_id {
            // we wanna go elsewhere to sell
            return ShipDecision::TravelTo(destination);
        }

        // sell here
        return ShipDecision::Sell(Commodity::Food, 100);
    }

    // should we go somewhere else to buy?
    let destination =
        market_calculations::calculate_where_to_buy_frakking_food(station_position, markets);
    let destination = destination.expect("No where to go");

    if &destination != station_id {
        return ShipDecision::TravelTo(destination);
    }

    //buy here!
    ShipDecision::Buy(Commodity::Food, 100)
}
