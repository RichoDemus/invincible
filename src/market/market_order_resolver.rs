use crate::market_calculations::{BuyOrder, SellOrder, Commodity, MarketOrder};
use uuid::Uuid;
use std::{cmp, mem};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct Transaction {
    pub seller: Uuid,
    pub buyer: Uuid,
    pub commodity: Commodity,
    pub amount: u64,
    pub price: u64,
}

#[cfg(test)]
impl Transaction {
    pub fn from(seller: Uuid, buyer: Uuid, amount:u64, price: u64) ->Self {
        Transaction {
            seller,
            buyer,
            commodity: Commodity::Food,
            amount,
            price
        }
    }
}

pub fn resolve_orders(mut old_orders: Vec<MarketOrder>, mut new_order: MarketOrder) -> (Vec<MarketOrder>, Vec<Transaction>) {
    let mut transactions = vec![];
    loop {
        let maybe_best_order = old_orders.iter_mut().enumerate()
            .filter(|(_, order)| {
                let left: MarketOrder = **order;
                let right: MarketOrder = new_order;
                // println!("{:?} disc: {:?} {:?} disc: {:?}", left, std::mem::discriminant(&left), right, std::mem::discriminant(&right));
                std::mem::discriminant(&left) != std::mem::discriminant(&right)
            })
            .filter(|(_, order)|order.commodity() == new_order.commodity())
            .fold1(|(left_index, left_order), (right_index, right_order)| {
                // println!("Folderino: {} {:?}  {} {:?}", left_index, left_order, right_index, right_order);
                if left_order.price() < right_order.price() {
                    (right_index, right_order)
                } else {
                    (left_index, left_order)
                }
            });

        if maybe_best_order.is_none() {
            // no order of the oposite type at all
            old_orders.push(new_order);
            return (old_orders, transactions);
        }

        // we have matching orders
        let (best_order_index, best_order) = maybe_best_order.unwrap();

        let amount_to_transfer = cmp::min(best_order.amount(), new_order.amount());
        if amount_to_transfer == 0 {
            panic!(format!("Attempted to do a zero amount transfer: {:?} {:?}", best_order, new_order));
        }

        let (seller, seller_price, buyer, buyer_price) = match (&best_order, &new_order) {
            (MarketOrder::SellOrder(seller), MarketOrder::BuyOrder(buyer)) => (seller.seller, seller.price, buyer.buyer, buyer.price),
            (MarketOrder::BuyOrder(buyer), MarketOrder::SellOrder(seller)) => (seller.seller, seller.price, buyer.buyer, buyer.price),
            _ => panic!("this shouldn't happen"),
        };

        if buyer_price < seller_price {
            // the highest buy price is lower than the lowest sell price
            old_orders.push(new_order);
            return (old_orders, vec![]);
        }

        new_order.reduce_amount(amount_to_transfer);
        best_order.reduce_amount(amount_to_transfer);

        transactions.push(Transaction{
            seller,
            buyer,
            commodity: Commodity::Food,
            amount: amount_to_transfer,
            price: best_order.price(), // take the price most beneficial to owner of new order
        });

        if best_order.amount() == 0 {
            old_orders.remove(best_order_index);
        }
        if new_order.amount() == 0 {
            // the new order is fully consumed, we done here
            return (old_orders, transactions);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::uuid;
    use std::cmp::Ordering;
    use itertools::Itertools;

    #[test]
    fn test_no_orders() {
        let (unresolved_orders, transactions) = resolve_orders(vec![], MarketOrder::SellOrder(SellOrder::from(uuid(0), 100, 10)));
        assert_eq!(unresolved_orders.len(), 1);
        assert!(transactions.is_empty());
    }

    #[test]
    fn should_not_do_anything_if_no_orders_match() {
        let old_orders = vec![
            MarketOrder::BuyOrder(BuyOrder::from(uuid(1), 100, 9)),
        ];
        let new_order = MarketOrder::SellOrder(SellOrder::from(uuid(0), 100, 10));

        let (unresolved_orders, transactions) = resolve_orders(old_orders, new_order);

        assert_eq!(unresolved_orders.len(), 2);
        assert!(transactions.is_empty());
    }


    #[test]
    fn should_return_one_transaction_for_basic_case() {
        let old_orders = vec![
            MarketOrder::BuyOrder(BuyOrder::from(uuid(1), 100, 10)),
        ];
        let new_order = MarketOrder::SellOrder(SellOrder::from(uuid(0), 100, 10));

        let (unresolved_orders, transactions) = resolve_orders(old_orders, new_order);

        assert!(unresolved_orders.is_empty());
        assert_eq!(transactions, vec![Transaction::from(uuid(0), uuid(1), 100, 10)]);
    }

    #[test]
    fn should_pick_oldest_order_if_same_price() {
        let old_orders = vec![
            MarketOrder::BuyOrder(BuyOrder::from(uuid(1), 100, 10)),
            MarketOrder::BuyOrder(BuyOrder::from(uuid(2), 100, 10)),
        ];
        let new_order = MarketOrder::SellOrder(SellOrder::from(uuid(0), 100, 10));

        let (unresolved_orders, transactions) = resolve_orders(old_orders, new_order);

        assert_eq!(unresolved_orders, vec![MarketOrder::BuyOrder(BuyOrder::from(uuid(2), 100, 10))]);
        assert_eq!(transactions, vec![Transaction::from(uuid(0), uuid(1), 100, 10)]);
    }

    #[test]
    fn should_partially_consume_existing_order() {
        let old_orders = vec![
            MarketOrder::BuyOrder(BuyOrder::from(uuid(1), 100, 10)),
        ];
        let new_order = MarketOrder::SellOrder(SellOrder::from(uuid(0), 50, 10));

        let (unresolved_orders, transactions) = resolve_orders(old_orders, new_order);

        assert_eq!(unresolved_orders, vec![MarketOrder::BuyOrder(BuyOrder::from(uuid(1), 50, 10))]);
        assert_eq!(transactions, vec![Transaction::from(uuid(0), uuid(1), 50, 10)]);
    }

    #[test]
    fn should_partially_consume_new_order() {
        let old_orders = vec![
            MarketOrder::BuyOrder(BuyOrder::from(uuid(1), 50, 10)),
        ];
        let new_order = MarketOrder::SellOrder(SellOrder::from(uuid(0), 100, 10));

        let (unresolved_orders, transactions) = resolve_orders(old_orders, new_order);

        assert_eq!(unresolved_orders, vec![MarketOrder::SellOrder(SellOrder::from(uuid(0), 50, 10))]);
        assert_eq!(transactions, vec![Transaction::from(uuid(0), uuid(1), 50, 10)]);
    }

    #[test]
    fn should_consume_multiple_orders_to_satisfy_new_order() {
        let old_orders = vec![
            MarketOrder::BuyOrder(BuyOrder::from(uuid(0), 10, 10)),
            MarketOrder::BuyOrder(BuyOrder::from(uuid(1), 20, 9)),
            MarketOrder::BuyOrder(BuyOrder::from(uuid(0), 10, 9)),
        ];
        let new_order = MarketOrder::SellOrder(SellOrder::from(uuid(2), 35, 5));

        let (unresolved_orders, transactions) = resolve_orders(old_orders, new_order);

        assert_eq!(unresolved_orders, vec![MarketOrder::BuyOrder(BuyOrder::from(uuid(0), 5, 9))]);
        assert_eq!(transactions, vec![
            Transaction::from(uuid(2), uuid(0), 10, 10),
            Transaction::from(uuid(2), uuid(1), 20, 9),
            Transaction::from(uuid(2), uuid(0), 5, 9),
        ]);
    }
}
