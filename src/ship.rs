use std::collections::HashMap;

use nalgebra::Point2;
use rand::seq::IteratorRandom;
use rand::Rng;
use uuid::Uuid;

use crate::components::{Resource, Stockpiles};
use crate::market_calculations::MarketWithPosition;

#[derive(Debug)]
pub enum ShipDecision {
    TravelTo(Uuid),
    Buy(Resource, u64),
    Sell(Resource, u64),
}

pub fn figure_out_what_to_do_in_space(
    _position: &Point2<f64>,
    _ship_inventory: &Stockpiles,
    markets: &HashMap<Uuid, MarketWithPosition>,
) -> ShipDecision {
    let mut rng = rand::thread_rng();
    ShipDecision::TravelTo(
        *markets
            .keys()
            .choose(&mut rng)
            .expect("there should be a station here"),
    )
}

pub fn figure_out_what_to_do_at_station(
    station_id: &Uuid,
    ship_inventory: &Stockpiles,
    markets: &HashMap<Uuid, MarketWithPosition>,
) -> ShipDecision {
    let mut rng = rand::thread_rng();
    if let Some(_station_stockpiles) = markets.get(station_id) {
        if let Some((resource, amount)) = ship_inventory.stockpiles.iter().next() {
            // sell something
            if rng.gen_bool(0.5) {
                ShipDecision::Sell(*resource, *amount)
            } else {
                ShipDecision::TravelTo(
                    *markets
                        .keys()
                        .choose(&mut rng)
                        .expect("there should be a station here"),
                )
            }
        } else {
            // ship empty but something
            let resource_to_buy = Resource::Food; //todo dont hardcode
            ShipDecision::Buy(resource_to_buy, 100)
        }
    } else {
        ShipDecision::TravelTo(
            *markets
                .keys()
                .choose(&mut rng)
                .expect("there should be a station here"),
        )
    }
}
