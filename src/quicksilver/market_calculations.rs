use std::{cmp, fmt};
use std::cmp::Ordering;
use std::fmt::Debug;

use itertools::Itertools;
use nalgebra::{Point2, Vector2};
use uuid::Uuid;
use quicksilver::log;
use crate::quicksilver::projections::id_to_name;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Commodity {
    Water, Food, Hydrogen
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MarketOrder {
    BuyOrder(BuyOrder),
    SellOrder(SellOrder)
}

impl MarketOrder {
    pub fn price(&self) -> u64 {
        match self {
            MarketOrder::BuyOrder(order) => order.price,
            MarketOrder::SellOrder(order) => order.price,
        }
    }
    pub fn amount(&self) -> u64 {
        match self {
            MarketOrder::BuyOrder(order) => order.amount,
            MarketOrder::SellOrder(order) => order.amount,
        }
    }
    pub fn commodity(&self) -> Commodity {
        match self {
            MarketOrder::BuyOrder(order) => order.commodity,
            MarketOrder::SellOrder(order) => order.commodity,
        }
    }
    pub fn reduce_amount(&mut self, amount:u64) {
        match self {
            MarketOrder::BuyOrder(order) => order.amount -= amount,
            MarketOrder::SellOrder(order) => order.amount -= amount,
        }
    }
    pub fn is_other_price_better(&self, other: &MarketOrder) -> bool {
        match (self, other) {
            (MarketOrder::BuyOrder(left_order), MarketOrder::BuyOrder(right_order)) => left_order.price < right_order.price,
            (MarketOrder::SellOrder(left_order), MarketOrder::SellOrder(right_order)) => left_order.price > right_order.price,
            _ => panic!("Tried to do price comparison of orders of different types"),
        }
    }

    pub fn owner(&self) -> Uuid {
        match self {
            MarketOrder::BuyOrder(order) => order.buyer,
            MarketOrder::SellOrder(order) => order.seller,
        }
    }
}

impl From<BuyOrder> for MarketOrder {
    fn from(order: BuyOrder) -> Self {
       MarketOrder::BuyOrder(order)
    }
}

impl From<SellOrder> for MarketOrder {
    fn from(order: SellOrder) -> Self {
        MarketOrder::SellOrder(order)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct BuyOrder {
    pub id: Uuid,
    pub commodity: Commodity,
    pub buyer: Uuid,
    pub location: Uuid,
    pub position: Point2<f64>,
    pub amount: u64,
    pub price: u64,
}
#[cfg(test)]
impl BuyOrder {
    pub fn from(buyer:Uuid, amount: u64, price:u64) -> MarketOrder {
        MarketOrder::BuyOrder(BuyOrder {
            id: Default::default(),
            commodity: Commodity::Water,
            buyer,
            location: Default::default(),
            position: Point2::new(0.,0.),
            amount,
            price,
        })
    }
    pub fn from_w_commodity(buyer:Uuid, amount: u64, price:u64, commodity:Commodity) -> MarketOrder {
        MarketOrder::BuyOrder(BuyOrder {
            id: Default::default(),
            commodity,
            buyer,
            location: Default::default(),
            position: Point2::new(0.,0.),
            amount,
            price,
        })
    }
}
impl Debug for BuyOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buy")
            .field("b", id_to_name(&self.buyer).value())
            .field("l", id_to_name(&self.location).value())
            .field("c", &self.commodity)
            .field("a", &self.amount)
            .field("p", &self.price)
            .finish()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct   SellOrder{
    pub id: Uuid,
    pub commodity: Commodity,
    pub seller: Uuid,
    pub location: Uuid,
    pub position: Point2<f64>,
    pub amount: u64,
    pub price: u64,
}

// #[cfg(test)]
// impl From<(Uuid, u64, u64)> for SellOrder {
//     fn from(from: (Uuid, u64, u64)) -> Self {
//         SellOrder {
//             id: Default::default(),
//             commodity: Commodity::Water,
//             seller: from.0,
//             location: Default::default(),
//             position: Point2::new(0.,0.),
//             amount: from.1,
//             price: from.2,
//         }
//     }
// }

#[cfg(test)]
impl SellOrder {
    pub fn from(seller:Uuid, amount: u64, price:u64) -> MarketOrder {
        MarketOrder::SellOrder(SellOrder {
            id: Default::default(),
            commodity: Commodity::Water,
            seller,
            location: Default::default(),
            position: Point2::new(0.,0.),
            amount,
            price,
        })
    }
    pub fn from_w_commodity(seller:Uuid, amount: u64, price:u64, commodity :Commodity) -> MarketOrder {
        MarketOrder::SellOrder(SellOrder {
            id: Default::default(),
            commodity,
            seller,
            location: Default::default(),
            position: Point2::new(0.,0.),
            amount,
            price,
        })
    }
}

impl Debug for SellOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sell")
            .field("s", id_to_name(&self.seller).value())
            .field("l", id_to_name(&self.location).value())
            .field("c", &self.commodity)
            .field("a", &self.amount)
            .field("p", &self.price)
            .finish()
    }
}
// }

pub fn calculate_basic_selling_price(
    stockpile_size: u64,
    max_stockpile: u64,
    _monthly_expenses: u64,
    _current_wallet: u64,
) -> u64 {
    let price_float = 30. * (1. - stockpile_size as f64 / max_stockpile as f64) + 5.;
    let result = price_float.round() as u64;
    result
}

pub fn calculate_basic_buying_price(
    stockpile_size: u64,
    max_stockpile: u64,
    _monthly_expenses: u64,
    _current_wallet: u64,
) -> u64 {
    let price_float = 30. * (1. - stockpile_size as f64 / max_stockpile as f64);
    price_float.round() as u64
}

pub fn get_sell_and_buy_price(
    current_amount: f64,
    target_amount: f64,
) -> (u64, u64) {
    // let buy_price_float:f64 = 30. * (1. - target_amount as f64 / current_amount as f64);
    let buy_price_float:f64 = 30. * target_amount / current_amount;
    let buy_price_float:f64 = buy_price_float.round();

    let sell_price_float:f64 = 30. * current_amount / target_amount;
    let sell_price_float:f64 = sell_price_float.round();


    (buy_price_float as u64, sell_price_float as u64)
}

//
// #[derive(Clone, Debug)]
// pub struct MarketWithPosition {
//     pub id: Uuid,
//     pub position: Point2<f64>,
//     pub food_buy_price: u64,
//     pub food_sell_price: u64,
// }
//
// #[derive(Clone, Debug)]
// pub struct Route {
//     pub source: (Uuid, Point2<f64>),
//     pub destination: Uuid,
//     pub commodity: Commodity,
// }
//
// #[allow(dead_code)]
// pub fn get_most_profitable_route(
//     markets: &[MarketWithPosition],
//     _current_position: &Point2<f64>,
// ) -> Route {
//     fn get_profits(source: &MarketWithPosition, destination: &MarketWithPosition) -> u64 {
//         destination
//             .food_buy_price
//             .saturating_sub(source.food_sell_price)
//     }
//     fn get_distance(source: &MarketWithPosition, destination: &MarketWithPosition) -> f64 {
//         let vector: Vector2<f64> = destination.position - source.position;
//
//         let distance: f64 = vector.magnitude();
//         distance.round()
//     }
//
//     fn get_profit_per_distance(profit: u64, distance: f64) -> f64 {
//         let profit = profit as f64;
//         let result = profit / distance;
//         if result.is_nan() {
//             panic!("profit/distance {}/{} failed", profit, distance);
//         }
//         result
//     }
//
//     let (source, source_position, destination, _profit, _distance, _profit_per_distance) = markets
//         .iter()
//         .permutations(2)
//         .map(|vec| {
//             let source = *vec.get(0).expect("should be here");
//             let destination = *vec.get(1).expect("here as well");
//             (source, destination)
//         })
//         .map(|(source, destination)| (source, destination, get_profits(source, destination)))
//         .map(|(source, destination, profit)| {
//             (
//                 source,
//                 destination,
//                 profit,
//                 get_distance(source, destination),
//             )
//         })
//         .map(|(source, destination, profit, distance)| {
//             (
//                 source,
//                 destination,
//                 profit,
//                 distance,
//                 get_profit_per_distance(profit, distance),
//             )
//         })
//         // .map(|(source, destination, profit, distance, profit_per_distance)|{
//         //     println!("route: {}->{} profit: {}, distance: {}, profit per distance: {}", source.id, destination.id, profit, distance, profit_per_distance);
//         //     (source, destination, profit, distance, profit_per_distance)
//         // })
//         .sorted_by(
//             |(_, _, _, _, left_profit_per_distance), (_, _, _, _, right_profit_per_distance)| {
//                 right_profit_per_distance
//                     .partial_cmp(left_profit_per_distance)
//                     .unwrap_or_else(|| {
//                         panic!(
//                             "Couldn't order {} and {}",
//                             left_profit_per_distance, right_profit_per_distance
//                         )
//                     })
//             },
//         )
//         .next()
//         .map(
//             |(source, destination, profit, distance, profit_per_distance)| {
//                 (
//                     source.id,
//                     source.position,
//                     destination.id,
//                     profit,
//                     distance,
//                     profit_per_distance,
//                 )
//             },
//         )
//         .expect("no routes?");
//
//     Route {
//         source: (source, source_position),
//         destination,
//         commodity: Commodity::Food, //don't hardcode ^^
//     }
// }

pub fn calculate_where_to_buy_frakking_food(
    position: &Point2<f64>,
    market_orders: &Vec<MarketOrder>,
) -> Option<Uuid> {
    let where_to_buy_food = market_orders
        .into_iter()
        .filter_map(|order| match order {
            MarketOrder::BuyOrder(_) => None,
            MarketOrder::SellOrder(order) => Some(order),
        })
        .map(|order| (order.location, order.position.clone(), order.price))
        .fold1(
            |(left_id, left_position, left_sell_price),
             (right_id, right_position, right_sell_price)| {
                match left_sell_price.cmp(&right_sell_price) {
                    Ordering::Less => (left_id, left_position, left_sell_price),
                    Ordering::Greater => (right_id, right_position, right_sell_price),
                    Ordering::Equal => {
                        // same sell price, go by distance
                        let left_distance = position - left_position;
                        let left_distance = left_distance.magnitude();

                        let right_distance = position - right_position;
                        let right_distance = right_distance.magnitude();

                        // println!("Same sell price: {}, distances: {:?} {} {:?} {}", left_sell_price, left_id, left_distance, right_id, right_distance);
                        if left_distance < right_distance {
                            (left_id, left_position, left_sell_price)
                        } else {
                            (right_id, right_position, right_sell_price)
                        }
                    }
                }
            },
        )
        .map(|(id, _position, _sell_price)| id);
    // if let Some(id) = where_to_buy_food {
    //     log::info!("Decided to buy food from: {}", id_to_name(&id).value());
    //     log::info!("Candidates:");
    //     for order in market_orders.iter().filter_map(|order| match order {
    //         MarketOrder::BuyOrder(_) => None,
    //         MarketOrder::SellOrder(order) => Some(order),
    //     }).sorted_by_key(|order| order.price) {
    //         log::info!("\torder: {:?}", order);
    //     }
    // }
    where_to_buy_food
}

pub fn calculate_where_to_sell_cargo(
    _position: &Point2<f64>,
    food_amount: u64,
    market_orders: &Vec<MarketOrder>,
) -> Option<Uuid> {
    market_orders
        .iter()
        .filter_map(|order| match order {
            MarketOrder::SellOrder(_) => None,
            MarketOrder::BuyOrder(order) => Some(order),
        })
        .map(|buy_order| {
            let buy_amount = buy_order.amount;
            let buy_amount = cmp::min(buy_amount, food_amount);
            let buy_price_per_unit = buy_order.price;
            let total_sell = buy_amount * buy_price_per_unit;

            (buy_order.location, total_sell)
        })
        .filter(|(_,amount)| amount > &0)
        .sorted_by(|(_, left_sell_value), (_, right_sell_value)| {
            let left_sell_value: &u64 = left_sell_value;
            let right_sell_value: u64 = *right_sell_value;
            right_sell_value
                .partial_cmp(left_sell_value)
                .expect("couldn't unwrap ordering")
        })
        .next()
        .map(|(id, _)| id)
}

pub fn create_buy_order(amount: u64, commodity: Commodity, buyer: Uuid, sell_orders: Vec<&SellOrder>, location:Uuid) -> BuyOrder {
    let mut amount_left = amount;
    let some_order = sell_orders.first().expect("should be an order here");
    let mut lowest_price = some_order.price;
    let position = some_order.position;
    for order in sell_orders.iter().sorted_by_key(|order| order.price) {
        lowest_price = order.price;
        amount_left = amount_left.saturating_sub(order.amount);
        if amount_left < 1 {
            break
        }
    }
    assert_ne!(amount, 0, "Don't create zero amount buy orders");
    BuyOrder {
        id: Uuid::new_v4(),
        commodity,
        buyer,
        location,
        position,
        amount,
        price: lowest_price
    }
}

pub fn create_sell_order(amount: u64, commodity: Commodity, seller: Uuid, buy_orders: Vec<&BuyOrder>, location:Uuid) -> SellOrder {
    println!("amount: {}", amount);
    let mut amount_left = amount;
    let some_order = buy_orders.first().expect("should be an order here");
    let mut highest_price = some_order.price;
    let position = some_order.position;
    for order in buy_orders.iter().sorted_by_key(|order| order.price).rev() {
        highest_price = order.price;
        amount_left = amount_left.saturating_sub(order.amount);
        if amount_left < 1 {
            break
        }
    }
    if amount == 0 {
        panic!("sell amount 0, buy orders: {:?}", buy_orders)
    }
    assert_ne!(amount, 0, "Don't create zero amount sell orders");
    SellOrder {
        id: Uuid::new_v4(),
        commodity,
        seller,
        location,
        position,
        amount,
        price: highest_price
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Point;

    use crate::quicksilver::util::uuid;

    use super::*;

    #[test]
    fn test_buy_and_sell_orders() {

        // let (buy_price, sell_price) = get_sell_and_buy_price(1000., 1000.);
    //

        let inputs = vec![
            (1000., 300.),
            (1000., 700.),
            (1000., 1000.),
            (1000., 1600.),
            (1000., 2000.),
        ];

        for (current_amount, target_amount) in inputs {
            let (buy_price, sell_price) = get_sell_and_buy_price(current_amount, target_amount);

            println!("If we have {} food and want to have {}. buy: {} sell {}", current_amount, target_amount, buy_price, sell_price);
        }

    }

//
//     #[test]
//     fn it_works() {
//         let source = MarketWithPosition {
//             id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
//             position: Point2::new(20., 20.),
//             food_buy_price: 2,
//             food_sell_price: 7,
//         };
//         let destination = MarketWithPosition {
//             id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
//             position: Point2::new(10., 10.),
//             food_buy_price: 10,
//             food_sell_price: 12,
//         };
//         let crappy_place = MarketWithPosition {
//             id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
//             position: Point2::new(20., 21.),
//             food_buy_price: 2,
//             food_sell_price: 20,
//         };
//         let good_but_to_far_away = MarketWithPosition {
//             id: Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap(),
//             position: Point2::new(200000., 20.),
//             food_buy_price: 1,
//             food_sell_price: 2,
//         };
//         let markets = vec![
//             destination.clone(),
//             source.clone(),
//             crappy_place,
//             good_but_to_far_away,
//         ];
//
//         let result = get_most_profitable_route(&markets, &Point2::new(15., 15.));
//
//         assert_eq!(result.source.0, source.id, "wrong source");
//         assert_eq!(result.destination, destination.id, "wrong destination");
//     }

    #[test]
    fn test_calc_where_to_buy_cargo() {
        let best = SellOrder{
            id: uuid(0),
            commodity: Commodity::Food,
            seller: uuid(10),
            location: uuid(20),
            position: Point2::new(1.,1.),
            amount: 100,
            price: 10,
        };
        let to_expensive = SellOrder{
            id: uuid(1),
            commodity: Commodity::Food,
            seller: uuid(11),
            location: uuid(21),
            position: Point2::new(1.,1.),
            amount: 100,
            price: 11,
        };
        let to_far_away = SellOrder{
            id: uuid(2),
            commodity: Commodity::Food,
            seller: uuid(12),
            location: uuid(22),
            position: Point2::new(100.,100.),
            amount: 100,
            price: 5,
        };

        let sell_orders = vec![to_expensive.into(), best.into(), to_far_away.into()];

        let result = calculate_where_to_buy_frakking_food(&Point2::new(0., 0.), &sell_orders);
        assert!(result.is_some());
        if let Some(id) = result {
            assert_eq!(
                id,
                to_far_away.location // todo should be best
            );
        }
    }

    #[test]
    fn test_calculate_where_to_sell_cargo() {
        let best = BuyOrder{
            id: uuid(0),
            commodity: Commodity::Food,
            buyer: uuid(10),
            location: uuid(20),
            position: Point2::new(1.,1.),
            amount: 100,
            price: 20,
        };
        let to_cheap = BuyOrder{
            id: uuid(1),
            commodity: Commodity::Food,
            buyer: uuid(11),
            location: uuid(21),
            position: Point2::new(1.,1.),
            amount: 100,
            price: 10,
        };
        let to_far_away = BuyOrder{
            id: uuid(2),
            commodity: Commodity::Food,
            buyer: uuid(12),
            location: uuid(22),
            position: Point2::new(100.,100.),
            amount: 100,
            price: 40,
        };

        let buy_orders = vec![best.into(), to_cheap.into(), to_far_away.into()];

        let result = calculate_where_to_sell_cargo(&Point2::new(0., 0.), 200, &buy_orders);

        assert!(result.is_some());
        if let Some(id) = result {
            assert_eq!(
                id,
                uuid(22) // todo should be 20
            );
        }
    }

    #[test]
    fn test_create_buy_order(){
        let cheapest = &SellOrder {
            id: Uuid::new_v4(),
            commodity: Commodity::Food,
            seller: Uuid::new_v4(),
            location: Uuid::new_v4(),
            position: Point2::new(0.,0.),
            amount: 50,
            price: 10,
        };
        let mid_tier = &SellOrder {
            amount:100,
            price: 20,
            ..cheapest.clone()
        };
        let expensive = &SellOrder {
            amount:100,
            price: 30,
            ..cheapest.clone()
        };

        let orders = vec![cheapest, mid_tier, expensive];

        let result = create_buy_order(100, Commodity::Food, uuid(0), orders, uuid(1));

        assert_eq!(result.price, 20);
        assert_eq!(result.amount, 100);
    }
}