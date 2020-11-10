use nalgebra::Point2;
use ncollide2d::shape::Ball;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Id {
    pub uuid: Uuid,
}

impl Default for Id {
    fn default() -> Self {
        Id {
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub point: Point2<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Planet {}

#[derive(Clone, Debug, PartialEq)]
pub struct Name {
    pub name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Commodity {
    Water,
    Food,
    #[allow(dead_code)]
    Hydrogen,
    Fuel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NaturalResources {
    pub resource: Commodity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InventoryItem {
    pub amount: u64,
    pub bought_for: u64,
}

#[derive(Clone, Debug)]
pub struct Inventory {
    pub contents: Vec<(Commodity, InventoryItem)>,
    pub storage_capacity: u64,
}

impl Inventory {
    pub fn size(&self) -> u64 {
        self.contents.iter().map(|(_, item)| item.amount).sum()
    }
    pub fn space_left(&self) -> u64 {
        self.storage_capacity - self.size()
    }
    pub fn add(&mut self, commodity: Commodity, amount: u64, bought_for: u64) {
        let has_item_for_this_price = self
            .contents
            .iter()
            .any(|(comm, item)| comm == &commodity && item.bought_for == bought_for);
        if has_item_for_this_price {
            for (commodity_inner, item) in self.contents.iter_mut() {
                if commodity_inner == &commodity && item.bought_for == bought_for {
                    item.amount += amount;
                }
            }
        } else {
            self.contents
                .push((commodity, InventoryItem { amount, bought_for }));
        }
    }
    pub fn remove(&mut self, commodity: &Commodity, mut amount: u64) {
        while amount > 0 {
            for (com, item) in self.contents.iter_mut() {
                if com == commodity && item.amount > 0 {
                    item.amount -= 1;
                    amount -= 1;
                }
            }
        }
    }
    pub fn get_amount(&self, commodity: &Commodity) -> u64 {
        self.contents
            .iter()
            .filter(|(com, _)| com == commodity)
            .map(|(_, item)| item.amount)
            .sum()
    }
    pub fn clean(&mut self) {
        // todo impl
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Selected;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Selectable;

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub shape: Ball<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Population {
    pub population: u64,
}
