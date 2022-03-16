use std::collections::HashMap;

use nalgebra::Point2;
use ncollide2d::shape::Ball;
use quicksilver::log;
use rand::prelude::StdRng;
use rand::Rng;
use uuid::Uuid;

use crate::quicksilver::core::Core;
use crate::quicksilver::inventory::Inventory;
use crate::quicksilver::market::market_order_resolver;
use crate::quicksilver::market::market_order_resolver::Transaction;
use crate::quicksilver::market_calculations::{BuyOrder, Commodity, MarketOrder, SellOrder};
use crate::quicksilver::selectability::{
    PositionAndShape, Selectable, SelectableAndPositionAndShape,
};
use crate::quicksilver::{market_calculations, HEIGHT, WIDTH};

pub struct Planet {
    pub id: Uuid,
    pub name: String,
    pub position: Point2<f64>,
    pub shape: Ball<f64>,
    pub population: u64,
    pub water: bool,
    pub hydrogen: bool,
    pub selected: bool,
    pub fuel_plant: bool,
    pub items: Inventory,
    pub market_orders: Vec<MarketOrder>,
}

impl Default for Planet {
    fn default() -> Self {
        Planet {
            id: Uuid::new_v4(),
            name: "".to_string(),
            position: Point2::new(1., 1.),
            shape: Ball::new(10.),
            population: 1,
            water: false,
            hydrogen: false,
            selected: false,
            fuel_plant: false,
            items: Inventory::with_food_and_capacity(500, 100000),
            market_orders: Default::default(),
        }
    }
}

impl Planet {
    pub fn random(id: Uuid, names: &mut Vec<&str>, rng: &mut StdRng) -> Self {
        let x = rng.gen_range(0.0..WIDTH as f64);
        let y = rng.gen_range(0.0..HEIGHT as f64);
        let name = names.pop().expect("no more planet names");
        let mut inventory = Inventory::with_capacity(1000);
        inventory.add(Commodity::Food, 700);
        Planet {
            id,
            name: String::from(name),
            position: Point2::new(x, y),
            shape: Ball::new(10.),
            population: rng.gen_range(1..10),
            water: rng.gen_range(0..4) == 0,
            hydrogen: rng.gen_range(0..4) == 0,
            fuel_plant: rng.gen_range(0..4) == 0,
            selected: false,
            items: inventory,
            market_orders: Default::default(),
        }
    }

    pub fn tick_day(&mut self) -> Vec<Transaction> {
        if self.water {
            self.items.add(Commodity::Food, 100);
        }
        if self.hydrogen {
            self.items.add(Commodity::Hydrogen, 10);
        }

        // consume food
        {
            let current_food = self.items.get(&Commodity::Food);

            if current_food < 1 {
                //starvation
                self.population = self.population.saturating_sub(1);
            } else {
                self.items.remove(Commodity::Food, self.population);
            }
        }

        update_market_orders(self)
    }

    pub fn days_until_starvation(&self) -> u64 {
        self.items
            .get(&Commodity::Food)
            .checked_div(self.population)
            .unwrap_or(0)
    }

    pub fn add_and_process_market_order(&mut self, order: MarketOrder) -> Vec<Transaction> {
        // assert_ne!(order.price(), 0, "Tried to put a market order with zero price");
        assert_ne!(
            order.amount(),
            0,
            "Tried to put a market order with zero amount: {:?}",
            order
        );

        let orders = std::mem::replace(&mut self.market_orders, vec![]);

        let (orders, transactions) = market_order_resolver::resolve_orders(orders, order);
        let _ = std::mem::replace(&mut self.market_orders, orders);
        transactions
    }

    pub fn num_buy_orders(&self) -> usize {
        self.market_orders
            .iter()
            .filter(|order| match order {
                MarketOrder::BuyOrder(_) => true,
                MarketOrder::SellOrder(_) => false,
            })
            .count()
    }
    pub fn num_sell_orders(&self) -> usize {
        self.market_orders
            .iter()
            .filter(|order| match order {
                MarketOrder::BuyOrder(_) => false,
                MarketOrder::SellOrder(_) => true,
            })
            .count()
    }
}

impl SelectableAndPositionAndShape for Planet {}

impl Selectable for Planet {
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

impl PositionAndShape for Planet {
    fn position_and_shape(&self) -> (Point2<f64>, Ball<f64>) {
        (self.position, self.shape)
    }
}

// todo check that this is correct
fn update_market_orders(planet: &mut Planet) -> Vec<Transaction> {
    // remove all orders placed by this planet
    let planet_id = planet.id;
    planet
        .market_orders
        .retain(|order| order.owner() != planet_id);

    // decide our desired range of each commodity and update market orders accordingly
    let food_required_to_last_one_year = planet.population * 365;

    adjust_market_orders(
        Commodity::Food,
        &mut planet.market_orders,
        planet.items.get(&Commodity::Food) as i64,
        food_required_to_last_one_year as i64,
    );

    let mut transactions = vec![];
    let desired_food: u64 = 800;
    let food_amount = planet.items.get(&Commodity::Food);
    let two_thirds_food = food_amount.checked_div(3).unwrap_or(0) * 3;
    let missing_food = desired_food.saturating_sub(food_amount);
    if missing_food == 0 {
        // not missing any food, make sure we have no buy orders
        log::info!("{} isn't missing food, remove buy orders", planet.name);
        let maybe_pos = planet.market_orders.iter().position(|order| match order {
            MarketOrder::BuyOrder(order) => {
                order.buyer == planet.id && order.commodity == Commodity::Food
            }
            MarketOrder::SellOrder(_) => false,
        });
        if let Some(pos) = maybe_pos {
            planet.market_orders.remove(pos);
        }
    } else {
        //missing food, remove current buy order and create a new one
        log::info!(
            "{} is missing {} food, cancel/create buy order",
            planet.name,
            missing_food
        );
        let planet_id = planet.id;
        planet.market_orders.retain(|order| {
            // we wanna remove our buy order for food
            match order {
                MarketOrder::BuyOrder(buy_order) => {
                    if buy_order.buyer == planet_id && buy_order.commodity == Commodity::Food {
                        false
                    } else {
                        true
                    }
                }
                MarketOrder::SellOrder(_) => true,
            }
        });

        let mut new_transactions =
            planet.add_and_process_market_order(MarketOrder::BuyOrder(BuyOrder {
                id: Uuid::new_v4(),
                commodity: Commodity::Food,
                buyer: planet.id,
                location: planet.id,
                position: planet.position.clone(),
                amount: missing_food,
                price: market_calculations::calculate_basic_buying_price(
                    food_amount,
                    desired_food,
                    0,
                    0,
                ),
            }));
        transactions.append(&mut new_transactions);
    }

    if planet.days_until_starvation() < 5 {
        // remove sell order
        let maybe_pos = planet.market_orders.iter().position(|order| match order {
            MarketOrder::SellOrder(order) => {
                order.seller == planet.id && order.commodity == Commodity::Food
            }
            MarketOrder::BuyOrder(_) => false,
        });
        if let Some(pos) = maybe_pos {
            planet.market_orders.remove(pos);
        }
    } else if two_thirds_food != 0 {
        //enough food to sell, adjust sell order
        let pos = planet
            .market_orders
            .iter()
            .filter_map(|order| match order {
                MarketOrder::BuyOrder(_) => None,
                MarketOrder::SellOrder(order) => Some(order),
            })
            .position(|order| order.seller == planet.id && order.commodity == Commodity::Food);
        match pos {
            None => {
                let mut new_transactions =
                    planet.add_and_process_market_order(MarketOrder::SellOrder(SellOrder {
                        id: Uuid::new_v4(),
                        commodity: Commodity::Food,
                        seller: planet.id,
                        location: planet.id,
                        position: planet.position.clone(),
                        amount: two_thirds_food,
                        price: market_calculations::calculate_basic_selling_price(
                            food_amount,
                            desired_food,
                            0,
                            0,
                        ),
                    }));
                transactions.append(&mut new_transactions);
            }
            Some(pos) => {
                let order = planet
                    .market_orders
                    .get_mut(pos)
                    .expect("Should be a thing here");
                if let MarketOrder::SellOrder(sell_order) = order {
                    sell_order.amount = two_thirds_food;
                    sell_order.price = market_calculations::calculate_basic_selling_price(
                        food_amount,
                        desired_food,
                        0,
                        0,
                    );
                }
            }
        }
    }
    transactions
}

fn calc_desired_food(pop: u64) -> u64 {
    100 * pop
}

fn adjust_market_orders(
    commodity: Commodity,
    market: &mut Vec<MarketOrder>,
    current_amount: i64,
    target_amount: i64,
) -> Vec<Transaction> {
    let missing_amount = target_amount - current_amount;

    let missing_ratio = 1. / (missing_amount as f64 / current_amount as f64);
    println!("missing ratio = {}", missing_ratio);
    let buy_price = 0;
    let sell_price = 0;

    println!(
        "food. amount: {}, target: {}, missing: {}. sell price: {}, buy_price_ {}",
        current_amount, target_amount, missing_amount, sell_price, buy_price
    );
    // panic!();

    // if missing_amount == 0 {
    //     // not missing any , kinda don't do anything
    //     // log::info!("{} isn't missing {:?}, remove buy orders", planet.name, commodity);
    //
    //     let maybe_pos = planet.market_orders.iter().position(|order| match order {
    //         MarketOrder::BuyOrder(order) => order.buyer == planet.id && order.commodity == Commodity::Food,
    //         MarketOrder::SellOrder(_) => false,
    //     });
    //     if let Some(pos) = maybe_pos {
    //         planet.market_orders.remove(pos);
    //     }
    // } else {
    //     //missing food, remove current buy order and create a new one
    //     log::info!("{} is missing {} food, cancel/create buy order", planet.name, missing_food);
    //     let planet_id = planet.id;
    //     planet.market_orders.retain(|order| {
    //         // we wanna remove our buy order for food
    //         match order {
    //             MarketOrder::BuyOrder(buy_order) => {
    //                 if buy_order.buyer == planet_id && buy_order.commodity == Commodity::Food {
    //                     false
    //                 } else {
    //                     true
    //                 }
    //             },
    //             MarketOrder::SellOrder(_) => true,
    //         }
    //     });
    //
    //     let mut new_transactions = planet.add_and_process_market_order(MarketOrder::BuyOrder(BuyOrder {
    //         id: Uuid::new_v4(),
    //         commodity: Commodity::Food,
    //         buyer: planet.id,
    //         location: planet.id,
    //         position: planet.position.clone(),
    //         amount: missing_food,
    //         price: market_calculations::calculate_basic_buying_price(food_amount, desired_food, 0,0),
    //     }));
    //     transactions.append(&mut new_transactions);
    // }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::quicksilver::util::uuid;

    use super::*;

    #[test]
    fn should_not_have_buy_order_if_sufficient_food() {
        let mut planet = Planet::default();
        planet.population = 5;
        planet.items.add(Commodity::Food, 800);
        update_market_orders(&mut planet);

        assert_eq!(planet.num_buy_orders(), 0)
    }

    #[test]
    fn should_put_buy_order_if_insufficient_food() {
        let mut planet = Planet::default();
        planet.population = 5;
        planet.items.add(Commodity::Food, 300);
        update_market_orders(&mut planet);

        assert!(!planet.market_orders.is_empty())
    }

    // #[test]
    // fn should_not_have_sell_order_if_low_food() {
    //     let mut planet = Planet::default();
    //     planet.population = 5;
    //     planet.items.add(Commodity::Food, 10);
    //     update_market_orders(&mut planet);
    //
    //     assert_eq!(planet.num_sell_orders(), 0)
    // }

    #[test]
    fn should_put_sell_order_if_sufficient_food() {
        let mut planet = Planet::default();
        planet.population = 5;
        planet.items.add(Commodity::Food, 800);
        update_market_orders(&mut planet);

        assert!(!planet.market_orders.is_empty())
    }
}
