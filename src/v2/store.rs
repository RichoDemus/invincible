use crate::v2::inventory::Inventory;
use crate::v2::commodity::Commodity;
use std::collections::HashMap;

use crate::v2::inventory::Amount;
use uuid::Uuid;

pub type Credits = u64;

#[derive(Debug)]
pub struct Receipt {
    pub commodity: Commodity,
    pub amount: Amount,
    pub price: Credits,
}

pub struct Store {
    pub id: Uuid,
    pub inventory: Inventory,
    pub magically_produces_food: bool,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            inventory: Inventory::default(),
            magically_produces_food: false,
        }
    }
}

impl Store {
    fn give(&mut self, commodity: Commodity, amount: Amount) {
        self.inventory.add(commodity, amount);
    }

    fn take(&mut self, commodity: &Commodity, amount: Amount) {
        let _ = self.inventory.take(commodity, amount);
    }

    pub fn buy_from_store(&mut self, commodity: Commodity, amount: Amount, price: Option<Credits>) -> Option<Receipt> {
        if self.inventory.get(&commodity) < amount {
            // Not enough of that commodity
            return None
        }
        match self.price_check_buy_from_store(&commodity) {
            None =>
                None,

            Some(store_price) => {
                if price.is_none() {
                    Some(Receipt {
                        commodity,
                        amount,
                        price: store_price,
                    })
                } else if price.unwrap() == store_price {
                    Some(Receipt {
                        commodity,
                        amount,
                        price: store_price,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn sell_to_store(&mut self, commodity: Commodity, amount: Amount, price: Option<Credits>) -> Option<Receipt> {
        match self.price_check_sell_to_store(&commodity) {
            None =>
                None,

            Some(store_price) => {
                if price.is_none() {
                    Some(Receipt {
                        commodity,
                        amount,
                        price: store_price,
                    })
                } else if price.unwrap() == store_price {
                    Some(Receipt {
                        commodity,
                        amount,
                        price: store_price,
                    })
                } else {
                    None
                }
            }
        }

    }

    pub fn price_check_buy_from_store(&self, commodity: &Commodity) ->  Option<Credits> {
        debug_assert_eq!(commodity, &Commodity::Food);
        if self.magically_produces_food {
            Some(2)
        } else {
            Some(5)
        }
    }

    pub fn price_check_sell_to_store(&self, commodity: &Commodity) -> Option<Credits>{
        debug_assert_eq!(commodity, &Commodity::Food);
        if self.magically_produces_food {
            Some(1)
        } else {
            Some(3)
        }
    }

    fn list(&self) ->  &HashMap<Commodity, Amount> {
        &self.inventory.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_store_should_be_empty() {
        let store = Store::default();
        assert!(store.list().is_empty())
    }

    #[test]
    fn test_give_and_take() {
        let mut store = Store::default();
        assert!(store.list().is_empty());

        store.give(Commodity::Food, 10);
        let mut expected = HashMap::new();
        expected.insert(Commodity::Food, 10u64);
        assert_eq!(store.list(), &expected);

        store.take(&Commodity::Food, 10);
        assert!(store.list().is_empty());
    }

    #[test]
    fn buy_some_food() {
        let mut store = Store {
          magically_produces_food: true,
            ..Store::default()
        };

        store.give(Commodity::Food, 100);

        let buy_price = store.price_check_buy_from_store(&Commodity::Food).expect("Should be able to buy food");
        assert!(buy_price > 0);

        let receipt = store.buy_from_store(Commodity::Food, 10, Some(buy_price)).expect("Store should've accepted this sale");

        assert_eq!(receipt.commodity, Commodity::Food);
        assert_eq!(receipt.amount, 10);
        assert_eq!(receipt.price, buy_price);
    }

    #[test]
    fn sell_some_food() {
        let mut store = Store {
            magically_produces_food: false,
            ..Store::default()
        };

        store.give(Commodity::Food, 100);

        let sell_price = store.price_check_sell_to_store(&Commodity::Food).expect("Should be able to sell food");
        assert!(sell_price > 0);

        let receipt = store.sell_to_store(Commodity::Food, 10, Some(sell_price)).expect("Store should've accepted this sale");

        assert_eq!(receipt.commodity, Commodity::Food);
        assert_eq!(receipt.amount, 10);
        assert_eq!(receipt.price, sell_price);
    }
}
