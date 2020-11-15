use crate::market_calculations::Commodity;
use std::collections::HashMap;
use std::cmp;

pub struct Inventory {
    pub items: HashMap<Commodity, u64>,
    pub capacity: u64,
}

impl Inventory {
    pub fn with_capacity(capacity: u64) -> Self {
        Inventory{
            items: HashMap::new(),
            capacity,
        }
    }

    pub fn add(&mut self, commodity: Commodity, amount: u64) {
        let amount = cmp::min(amount, self.space_left());
        *self.items.entry(commodity).or_insert(0) += amount;
    }

    pub fn remove(&mut self, commodity: Commodity, amount: u64) {
        let current_food = *self.items.get(&commodity).unwrap_or(&0);
        let new_food = current_food.checked_sub(amount);
        if new_food.is_none() {
            println!("Warn: less than 0 {:?}", commodity);
        }
        self.items.insert(commodity, new_food.unwrap_or(0));
    }

    pub fn get(&self, commodity: &Commodity) -> u64 {
        *self.items.get(commodity).unwrap_or(&0)
    }

    pub fn space_left(&self) -> u64 {
        let size:u64 = self.items.values().sum();
        self.capacity - size
    }
}
