use crate::market_calculations::{BuyOrder, SellOrder, Commodity};
use uuid::Uuid;
use std::cmp;

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

pub fn resolve_orders(mut sell_orders: Vec<SellOrder>, mut buy_orders: Vec<BuyOrder>) -> (Vec<SellOrder>, Vec<BuyOrder>, Vec<Transaction>) {
    let mut transactions = vec![];

    let lowest_sell_order = sell_orders.last_mut();
    let highest_buy_order = buy_orders.last_mut();
    if lowest_sell_order.is_none() || highest_buy_order.is_none() {
        // we're out of buy or sell orders
        return (sell_orders, buy_orders, transactions);
    }

    let lowest_sell_order = lowest_sell_order.unwrap();
    let highest_buy_order = highest_buy_order.unwrap();

    if highest_buy_order.price < lowest_sell_order.price {
        // the highest buy price is lower than the lowest sell price
        return (sell_orders, buy_orders, transactions);
    }

    let amount_to_transfer = cmp::min(highest_buy_order.amount, lowest_sell_order.amount);
    if amount_to_transfer == 0 {
        panic!(format!("Attempted to do a zero amount transfer: {:?} {:?}", lowest_sell_order, highest_buy_order));
    }

    if highest_buy_order.commodity != lowest_sell_order.commodity {
        panic!("Opps, only support food buy orders for now");
    }

    // kinda enough info to create a transaction for now
    transactions.push(Transaction{
        seller: lowest_sell_order.seller,
        buyer: highest_buy_order.buyer,
        commodity: Commodity::Food,
        amount: amount_to_transfer,
        price: lowest_sell_order.price, // todo how to figure out price?
    });

    buy_orders.pop();
    sell_orders.pop();

    (sell_orders, buy_orders, transactions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::uuid;
    use std::cmp::Ordering;
    use itertools::Itertools;

    #[test]
    fn test_no_orders() {
        let (new_sell_orders, new_buy_orders, transactions) = resolve_orders(vec![], vec![]);
        assert!(new_sell_orders.is_empty());
        assert!(new_buy_orders.is_empty());
        assert!(transactions.is_empty());
    }

    #[test]
    fn should_not_do_anything_if_no_orders_match() {
        let sell_orders = vec![SellOrder::from(uuid(0), 100, 10)];
        let buy_orders = vec![BuyOrder::from(uuid(1), 100, 9)];

        let (new_sell_orders, new_buy_orders, transactions) = resolve_orders(sell_orders, buy_orders);

        assert_eq!(new_sell_orders.len(), 1);
        assert_eq!(new_buy_orders.len(), 1);
        assert!(transactions.is_empty());
    }

    #[test]
    fn should_return_one_transaction_for_basic_case() {
        let sell_orders = vec![SellOrder::from(uuid(0), 100, 10)];
        let buy_orders = vec![BuyOrder::from(uuid(1), 100, 10)];

        let (new_sell_orders, new_buy_orders, transactions) = resolve_orders(sell_orders, buy_orders);

        assert!(new_sell_orders.is_empty());
        assert!(new_buy_orders.is_empty());
        assert_eq!(transactions, vec![Transaction::from(uuid(0), uuid(1), 100, 10)]);
    }

    #[test]
    fn should_resolve_matching_buy_sell_orders() {
        // Make sure we consume the highest buy orders and lowest sell orders first

        let seller1 = uuid(0);
        let seller2 = uuid(1);
        let buyer1 = uuid(2);
        let buyer2 = uuid(3);


        let mut sell_orders = vec![];
        // sell_orders.push((seller1, 10, 2).into()); // this should be consumed first
        // sell_orders.push((seller2, 10, 3).into()); // should be consumed before the other price 2, since its posted earlier
        // sell_orders.push((seller1, 20, 3).into());
        // sell_orders.push((seller2, 100, 10).into()); //consumed last

        let mut buy_orders = vec![];
        // buy_orders.push((buyer1, 5, 1).into());
        // buy_orders.push((buyer2, 5, 1).into());


        // buy_orders.push((buyer2, 5, 1).into());

        resolve_orders(sell_orders, buy_orders);
    }

    #[test]
    fn test_test_test() {
        let mut orders = vec![];
        orders.push((0, 10));
        orders.push((1, 10));
        orders.push((2, 30));
        orders.push((3, 40));
        orders.push((4, 5));
        orders.push((5, 10));

        orders.sort_by(|left, right|{
            if left.1 < right.1 {
                Ordering::Less
            } else if left.1 == right.1 {
                left.0.cmp(&right.0).reverse()
            } else {
                Ordering::Greater
            }
        });
        // orders.reverse();

        println!("sorted:");
        for order in orders {
            println!("\t: {:?}", order);
        }


    }


    #[test]
    fn anders(){

        // let mut order_queue = vec![];
        //
        //
        //
        // let highest_buy_order: (index, order) = order_queue.enumerate().filter(isBuyOrder).fold1(|acc, other_order| take_highest_price(acc, other_order));
        // let lowest_sell_order: (index, order) = order_queue.enumerate().filter(isSellOrder).fold1(|(i, acc), other_order| take_lowest(acc, other_order));
        //
        // let (highest_buy_order, lowest_sell_order) = order_queue.enumerate().very_fancy_fold();

        let (anderses, svenses) = vec!["anders","anders","sven","anders","sven"].into_iter()
            .fold((vec![], vec![]),|(mut anderses, mut svenses): (Vec<&str>, Vec<&str>), next| {
                if next == "anders" {
                    if svenses.is_empty() {
                        anderses.push(next);
                    } else {
                        svenses.pop();
                    }
                } else {
                    todo!()
                }
                (anderses, svenses)
            });

        //
        //
        // // find x and y
        // let highest_buy_order = order_queue.get_highest_buy_order_price();
        // let sell = order_queue.iter().indexOf(item.price == highest_buy_order);
        //
        // let lowest_sell_order = order_queue.get_mut(x);
        // let highest_buy_order = order_queue.get_mut(y);


    }
}
