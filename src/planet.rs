use std::collections::HashMap;

use nalgebra::Point2;
use ncollide2d::shape::Ball;
use rand::prelude::StdRng;
use rand::Rng;
use uuid::Uuid;

use crate::{HEIGHT, WIDTH, market_calculations};
use crate::market_calculations::{Commodity, BuyOrder, SellOrder};
use crate::selectability::{Selectable, PositionAndShape, SelectableAndPositionAndShape};
use crate::core::Core;
use crate::inventory::Inventory;


pub struct Planet {
    pub id: Uuid,
    pub name: String,
    pub position: Point2<f64>,
    pub shape: Ball<f64>,
    pub population: u64,
    pub water: bool,
    pub selected: bool,
    pub items: Inventory,
    pub buy_orders: Vec<BuyOrder>,
    pub sell_orders: Vec<SellOrder>,
}

impl Default for Planet {
    fn default() -> Self {
        Planet {
            id: Default::default(),
            name: "".to_string(),
            position: Point2::new(1., 1.),
            shape: Ball::new(10.),
            population: 0,
            water: false,
            selected: false,
            items: Inventory { items: Default::default(), capacity: 1000 },
            buy_orders: Default::default(),
            sell_orders: Default::default(),
        }
    }
}

impl Planet {
    pub fn random(id: Uuid, names: &mut Vec<&str>, rng: &mut StdRng) -> Self {
        let x = rng.gen_range(0., WIDTH as f64);
        let y = rng.gen_range(0., HEIGHT as f64);
        let name = names.pop().expect("no more planet names");
        let mut inventory = Inventory::with_capacity(1000);
        inventory.add(Commodity::Food, 1000);
        Planet {
            id,
            name: String::from(name),
            position: Point2::new(x, y),
            shape: Ball::new(10.),
            population: rng.gen_range(1, 10),
            water: rng.gen_range(0, 4) == 0,
            selected: false,
            items: inventory,
            buy_orders: Default::default(),
            sell_orders: Default::default(),
        }
    }

    pub fn tick_day(&mut self) {
        if self.water {
            self.items.add(Commodity::Food, 100);
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

        update_market_orders(self);
    }

    pub fn days_until_starvation(&self) ->u64 {
        self.items.get(&Commodity::Food).checked_div(self.population).unwrap_or(0)
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


fn update_market_orders(planet:&mut Planet) {
    let desired_food:u64 = 800;
    let food_amount = planet.items.get(&Commodity::Food);
    let two_thirds_food = food_amount.checked_div(3).unwrap_or(0) * 3;
    let missing_food = desired_food.saturating_sub(food_amount);
    if missing_food == 0 {
        // not missing any food, make sure we have no buy orders
        // todo better way to remove 1 element
        planet.buy_orders = planet.buy_orders.iter()
            .cloned()
            .filter(|order|order.buyer == planet.id && order.commodity == Commodity::Food)
            .collect();
    } else {
        //missing food, adjust buy order
        let pos = planet.buy_orders.iter().position(|order|order.buyer == planet.id && order.commodity == Commodity::Food);
        match pos {
            None => {
                planet.buy_orders.push(BuyOrder {
                    id: Uuid::new_v4(),
                    commodity: Commodity::Food,
                    buyer: planet.id,
                    amount: missing_food,
                    price: market_calculations::calculate_basic_buying_price(food_amount, desired_food, 0,0),
                })
            }
            Some(pos) => {
                let order = planet.buy_orders.get_mut(pos).expect("Should be a thing here");
                order.amount = missing_food;
                order.price = market_calculations::calculate_basic_buying_price(food_amount, desired_food, 0,0);
            }
        }
    }

    if planet.days_until_starvation() < 5 {
        //remove sell order
        planet.sell_orders = planet.sell_orders.iter()
            .cloned()
            .filter(|order|order.seller == planet.id && order.commodity == Commodity::Food)
            .collect();
    } else {
        //enough food to sell, adjust sell order
        let pos = planet.sell_orders.iter().position(|order|order.seller == planet.id && order.commodity == Commodity::Food);
        match pos {
            None => {
                planet.sell_orders.push(SellOrder {
                    id: Uuid::new_v4(),
                    commodity: Commodity::Food,
                    seller: planet.id,
                    amount: two_thirds_food,
                    price: market_calculations::calculate_basic_buying_price(food_amount, desired_food, 0,0),
                })
            }
            Some(pos) => {
                let order = planet.sell_orders.get_mut(pos).expect("Should be a thing here");
                order.amount = two_thirds_food;
                order.price = market_calculations::calculate_basic_selling_price(food_amount, desired_food, 0,0);
            }
        }

    }
}

fn calc_desired_food(pop:u64) -> u64 {
    100 * pop
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_have_buy_order_if_sufficient_food() {
        let mut planet = Planet::default();
        planet.population = 5;
        planet.items.add(Commodity::Food, 800);
        update_market_orders(&mut planet);

        assert!(planet.buy_orders.is_empty())
    }

    #[test]
    fn should_put_buy_order_if_insufficient_food() {
        let mut planet = Planet::default();
        planet.population = 5;
        planet.items.add(Commodity::Food, 300);
        update_market_orders(&mut planet);

        assert!(!planet.buy_orders.is_empty())
    }

    #[test]
    fn should_not_have_sell_order_if_low_food() {
        let mut planet = Planet::default();
        planet.population = 5;
        planet.items.add(Commodity::Food, 10);
        update_market_orders(&mut planet);

        assert!(planet.sell_orders.is_empty())
    }

    #[test]
    fn should_put_sell_order_if_sufficient_food() {
        let mut planet = Planet::default();
        planet.population = 5;
        planet.items.add(Commodity::Food, 800);
        update_market_orders(&mut planet);

        assert!(!planet.sell_orders.is_empty())
    }
}