use crate::v2::inventory::Inventory;
use crate::v2::commodity::Commodity;
use std::collections::HashMap;

use crate::v2::inventory::Amount;

pub type Credits = u64;

struct Receipt {
    commodity: Commodity,
    amount: Amount,
    price: Credits,
}

#[derive(Default)]
struct Store {
    inventory: Inventory,
    magically_produces_food: bool,
}

impl Store {
    fn give(&mut self, commodity: Commodity, amount: Amount) {
        self.inventory.add(commodity, amount);
    }

    fn take(&mut self, commodity: &Commodity, amount: Amount) {
        let _ = self.inventory.take(commodity, amount);
    }

    fn buy_from_store(&mut self, commodity: Commodity, amount: Amount, price: Credits) -> Option<Receipt> {
        match self.price_check_buy_from_store(&commodity) {
            None =>
                None,

            Some(store_price) => {
                if price == store_price {
                    Some(Receipt {
                        commodity,
                        amount,
                        price,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn sell_to_store(&mut self, commodity: Commodity, amount: Amount, price: Credits) -> Option<Receipt> {
        match self.price_check_sell_to_store(&commodity) {
            None =>
                None,

            Some(store_price) => {
                if price == store_price {
                    Some(Receipt {
                        commodity,
                        amount,
                        price,
                    })
                } else {
                    None
                }
            }
        }

    }

    fn price_check_buy_from_store(&self, commodity: &Commodity) ->  Option<Credits> {
        debug_assert_eq!(commodity, &Commodity::Food);
        if self.magically_produces_food {
            Some(2)
        } else {
            None
        }
    }

    fn price_check_sell_to_store(&self, commodity: &Commodity) -> Option<Credits>{
        debug_assert_eq!(commodity, &Commodity::Food);
        if self.magically_produces_food {
            None
        } else {
            Some(1)
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

        assert!(store.price_check_sell_to_store(&Commodity::Food).is_none());

        store.give(Commodity::Food, 100);

        let buy_price = store.price_check_buy_from_store(&Commodity::Food).expect("Should be able to buy food");
        assert!(buy_price > 0);

        let receipt = store.buy_from_store(Commodity::Food, 10, buy_price).expect("Store should've accepted this sale");

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

        let receipt = store.sell_to_store(Commodity::Food, 10, sell_price).expect("Store should've accepted this sale");

        assert_eq!(receipt.commodity, Commodity::Food);
        assert_eq!(receipt.amount, 10);
        assert_eq!(receipt.price, sell_price);
    }
}
