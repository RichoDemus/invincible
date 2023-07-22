use std::collections::HashMap;

use bevy::prelude::*;
use itertools::Itertools;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::v2::commodity::Commodity;
use crate::v2::commodity::Commodity::{Food, Fuel, HydrogenTanks};
use crate::v2::inventory::Amount;
use crate::v2::inventory::Inventory;

pub type Credits = u64;

#[derive(Debug)]
pub struct Receipt {
    pub commodity: Commodity,
    pub amount: Amount,
    pub price: Credits,
}

#[derive(Debug)]
pub struct StoreListing {
    pub commodity: Commodity,
    pub amount: Amount,
    pub price: Credits,
}

#[derive(Component)]
pub struct Store {
    pub id: Uuid,
    pub inventory: Inventory,
}

impl Default for Store {
    fn default() -> Self {
        let mut store = Self {
            id: Uuid::new_v4(),
            inventory: Inventory::default(),
        };
        store.give(Commodity::Food, 100);
        store
    }
}

impl Store {
    pub fn give(&mut self, commodity: Commodity, amount: Amount) {
        self.inventory.add(commodity, amount);
    }

    pub fn take(&mut self, commodity: Commodity, amount: Amount) {
        let _ = self.inventory.take(&commodity, amount);
    }

    pub fn buy_from_store(
        &mut self,
        commodity: Commodity,
        amount: Amount,
        price: Option<Credits>,
    ) -> Option<Receipt> {
        if self.inventory.get(&commodity) < amount {
            info!("Not enough {:?}", commodity);
            return None;
        }
        match self.price_check_buy_specific_from_store(commodity) {
            None => {
                info!("Store doesn't sell {:?}", commodity);
                None
            }

            Some(store_price) => {
                if price.is_none() || price.unwrap() == store_price.price {
                    self.take(commodity, amount);
                    Some(Receipt {
                        commodity,
                        amount,
                        price: store_price.price,
                    })
                } else {
                    info!("Store doesn't sell {:?} for that price", commodity);
                    None
                }
            }
        }
    }

    pub fn sell_to_store(
        &mut self,
        commodity: Commodity,
        amount: Amount,
        price: Option<Credits>,
    ) -> Option<Receipt> {
        match self.price_check_sell_specific_to_store(commodity) {
            None => {
                info!("Store doesn't want to buy {:?}", commodity);
                None
            }

            Some(store_price) => {
                if price.is_none() || price.unwrap() == store_price.price {
                    self.give(commodity, amount);
                    Some(Receipt {
                        commodity,
                        amount,
                        price: store_price.price,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn price_check_buy_specific_from_store(
        &self,
        commodity: Commodity,
    ) -> Option<StoreListing> {
        let amount_stockpiled = self.inventory.get(&commodity);
        let max_sellable = amount_stockpiled.max(20);
        if amount_stockpiled > 200 {
            Some(StoreListing {
                commodity,
                amount: max_sellable,
                price: 1,
            })
        } else if amount_stockpiled > 100 {
            Some(StoreListing {
                commodity,
                amount: max_sellable,
                price: 2,
            })
        } else if amount_stockpiled < 30 {
            None
        } else {
            Some(StoreListing {
                commodity,
                amount: max_sellable,
                price: 5,
            })
        }
    }

    pub fn price_check_sell_specific_to_store(&self, commodity: Commodity) -> Option<StoreListing> {
        let amount_stockpiled = self.inventory.get(&commodity);
        if amount_stockpiled < 20 {
            Some(StoreListing {
                commodity,
                amount: 400, // todo make smarter
                price: 10,
            })
        } else if amount_stockpiled > 200 {
            None
        } else if amount_stockpiled > 100 {
            Some(StoreListing {
                commodity,
                amount: 400, // todo make smarter
                price: 1,
            })
        } else {
            Some(StoreListing {
                commodity,
                amount: 400, // todo make smarter
                price: 3,
            })
        }
    }

    // todo list same commodity multiple times for different prices based on inventory
    pub fn price_check_buy_from_store(&self) -> Vec<StoreListing> {
        Commodity::iter()
            .into_iter()
            .filter_map(|commodity| self.price_check_buy_specific_from_store(commodity))
            .collect()
    }

    pub fn price_check_sell_to_store(&self) -> Vec<StoreListing> {
        Commodity::iter()
            .into_iter()
            .filter_map(|commodity| self.price_check_sell_specific_to_store(commodity))
            .collect()
    }

    fn list(&self) -> &HashMap<Commodity, Amount> {
        &self.inventory.items
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn new_store_should_be_empty() {
//         let store = Store::default();
//         assert!(store.list().is_empty())
//     }
//
//     #[test]
//     fn test_give_and_take() {
//         let mut store = Store::default();
//         assert!(store.list().is_empty());
//
//         store.give(Commodity::Food, 10);
//         let mut expected = HashMap::new();
//         expected.insert(Commodity::Food, 10u64);
//         assert_eq!(store.list(), &expected);
//
//         store.take(&Commodity::Food, 10);
//         assert!(store.list().is_empty());
//     }
//
//     #[test]
//     fn buy_some_food() {
//         let mut store = Store {
//             magically_produces_food: true,
//             ..Store::default()
//         };
//
//         store.give(Commodity::Food, 100);
//
//         let buy_price = store
//             .price_check_buy_specific_from_store(&Commodity::Food)
//             .expect("Should be able to buy food");
//         assert!(buy_price > 0);
//
//         let receipt = store
//             .buy_from_store(Commodity::Food, 10, Some(buy_price))
//             .expect("Store should've accepted this sale");
//
//         assert_eq!(receipt.commodity, Commodity::Food);
//         assert_eq!(receipt.amount, 10);
//         assert_eq!(receipt.price, buy_price);
//     }
//
//     #[test]
//     fn sell_some_food() {
//         let mut store = Store {
//             magically_produces_food: false,
//             ..Store::default()
//         };
//
//         store.give(Commodity::Food, 100);
//
//         let sell_price = store
//             .price_check_sell_specific_to_store(&Commodity::Food)
//             .expect("Should be able to sell food");
//         assert!(sell_price > 0);
//
//         let receipt = store
//             .sell_to_store(Commodity::Food, 10, Some(sell_price))
//             .expect("Store should've accepted this sale");
//
//         assert_eq!(receipt.commodity, Commodity::Food);
//         assert_eq!(receipt.amount, 10);
//         assert_eq!(receipt.price, sell_price);
//     }
// }
