use std::collections::HashMap;
use std::ops::Not;

use bevy::prelude::*;

use crate::v2::commodity::Commodity;

pub(crate) type Amount = u64;

#[derive(Component)]
pub struct Inventory {
    pub items: HashMap<Commodity, Amount>,
    pub capacity: Amount,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            capacity: 100,
        }
    }
}

impl Inventory {
    pub fn with_food_and_capacity(food: Amount, capacity: Amount) -> Self {
        let mut items = HashMap::new();
        items.insert(Commodity::Food, food);
        Inventory { items, capacity }
    }
    pub fn with_capacity(capacity: Amount) -> Self {
        Inventory {
            items: HashMap::new(),
            capacity,
        }
    }

    pub fn add(&mut self, commodity: Commodity, amount: Amount) {
        // let amount = cmp::min(amount, self.space_left());
        *self.items.entry(commodity).or_insert(0) += amount;
    }

    pub fn take(&mut self, commodity: &Commodity, amount: Amount) -> Amount {
        debug_assert_ne!(amount, 0);
        if self.items.contains_key(commodity).not() {
            return 0;
        }

        if amount >= self.get(commodity) {
            let everything = self.get(commodity);
            self.items.remove(commodity);
            return everything;
        }

        *self.items.get_mut(commodity).unwrap() -= amount;

        amount
    }

    pub fn get(&self, commodity: &Commodity) -> Amount {
        *self.items.get(commodity).unwrap_or(&0)
    }

    pub fn discard(&mut self, commodity: Commodity) {
        *self.items.entry(commodity).or_insert(0) = 0;
    }

    pub fn space_left(&self) -> Amount {
        let size: Amount = self.items.values().sum();
        self.capacity - size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_inventory_should_be_empty() {
        let mut inventory = Inventory::with_capacity(10);

        assert_eq!(inventory.space_left(), 10);
        assert_eq!(inventory.get(&Commodity::Food), 0);
        assert_eq!(inventory.take(&Commodity::Food, 3), 0);
    }

    #[test]
    fn add_remove_commodities() {
        let mut inventory = Inventory::with_capacity(10);
        inventory.add(Commodity::Food, 3);

        assert_eq!(inventory.space_left(), 7);
        assert_eq!(inventory.get(&Commodity::Food), 3);
        assert_eq!(inventory.take(&Commodity::Food, 5), 3);
    }

    #[test]
    fn remove_not_all_commodities() {
        let mut inventory = Inventory::with_capacity(10);
        inventory.add(Commodity::Food, 10);

        assert_eq!(inventory.take(&Commodity::Food, 5), 5);
        assert_eq!(inventory.get(&Commodity::Food), 5);
        assert_eq!(inventory.space_left(), 5);

        assert_eq!(inventory.take(&Commodity::Food, 1), 1);
        assert_eq!(inventory.get(&Commodity::Food), 4);
        assert_eq!(inventory.space_left(), 6);

        assert_eq!(inventory.take(&Commodity::Food, 4), 4);
        assert_eq!(inventory.get(&Commodity::Food), 0);
        assert_eq!(inventory.space_left(), 10);
    }
}
