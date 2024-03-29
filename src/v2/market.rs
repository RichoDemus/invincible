use uuid::Uuid;

use crate::v2::commodity::Commodity;
use crate::v2::store::{Credits, Store};

#[derive(Debug)]
pub struct CommodityListing {
    pub store: Uuid,
    pub commodity: Commodity,
    pub price: Credits,
}

#[derive(Default)]
pub struct Market {
    stores: Vec<Store>,
}

impl Market {
    pub fn add_store(&mut self, store: Store) {
        self.stores.push(store);
    }

    pub fn get_sellers(&self, commodity: Commodity) -> Vec<CommodityListing> {
        self.stores
            .iter()
            .filter_map(|store| {
                store
                    .price_check_buy_specific_from_store(commodity)
                    .map(|price| CommodityListing {
                        store: store.id,
                        commodity,
                        price,
                    })
            })
            .collect()
    }

    pub fn get_buyers(&self, commodity: Commodity) -> Vec<CommodityListing> {
        self.stores
            .iter()
            .filter_map(|store| {
                store
                    .price_check_sell_specific_to_store(&commodity)
                    .map(|price| CommodityListing {
                        store: store.id,
                        commodity,
                        price,
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_profitable_trade() {
        let mut market = Market::default();
        market.add_store(Store {
            magically_produces_food: false,
            ..Store::default()
        });
        market.add_store(Store {
            magically_produces_food: true,
            ..Store::default()
        });

        let cheapest_place_to_buy_food = market
            .get_sellers(Commodity::Food)
            .into_iter()
            .min_by_key(|listing| listing.price)
            .expect("Should be one store here");

        let expensivest_place_to_sell_food = market
            .get_buyers(Commodity::Food)
            .into_iter()
            .max_by_key(|listing| listing.price)
            .expect("Should be one store here");

        assert_ne!(
            cheapest_place_to_buy_food.store,
            expensivest_place_to_sell_food.store
        );
        assert!(
            cheapest_place_to_buy_food.price < expensivest_place_to_sell_food.price,
            "{:?} should be cheaper than {:?}",
            cheapest_place_to_buy_food,
            expensivest_place_to_sell_food
        );
        let profit = expensivest_place_to_sell_food.price - cheapest_place_to_buy_food.price;
        assert!(profit > 0);
    }
}
