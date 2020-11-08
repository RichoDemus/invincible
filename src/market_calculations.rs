use itertools::Itertools;
use nalgebra::{Point2, Vector2};
use uuid::Uuid;

pub fn calculate_basic_selling_price(
    stockpile_size: u64,
    max_stockpile: u64,
    _monthly_expenses: u64,
    _current_wallet: u64,
) -> u64 {
    let price_float = 30. * (1. - stockpile_size as f64 / max_stockpile as f64) + 5.;
    price_float.round() as u64
}

pub fn calculate_basic_buying_price(
    stockpile_size: u64,
    max_stockpile: u64,
    _monthly_expenses: u64,
    _current_wallet: u64,
) -> u64 {
    let price_float = 30. * (1. - stockpile_size as f64 / max_stockpile as f64);
    price_float.round() as u64
}

#[derive(Clone, Debug)]
pub struct MarketWithPosition {
    pub id: Uuid,
    pub position: Point2<f64>,
    pub food_buy_price: u64,
    pub food_sell_price: u64,
}

pub struct Route {
    pub source: (Uuid, Point2<f64>),
    pub destination: Uuid,
}

pub fn get_most_profitable_route(
    markets: &[MarketWithPosition],
    _current_position: &Point2<f64>,
) -> Route {
    fn get_profits(source: &MarketWithPosition, destination: &MarketWithPosition) -> u64 {
        destination
            .food_buy_price
            .saturating_sub(source.food_sell_price)
    }
    fn get_distance(source: &MarketWithPosition, destination: &MarketWithPosition) -> f64 {
        let vector: Vector2<f64> = destination.position - source.position;

        let distance: f64 = vector.magnitude();
        distance.round()
    }

    fn get_profit_per_distance(profit: u64, distance: f64) -> f64 {
        let profit = profit as f64;
        let result = profit / distance;
        if result.is_nan() {
            panic!("profit/distance {}/{} failed", profit, distance);
        }
        result
    }

    let (source, source_position, destination, _profit, _distance, _profit_per_distance) = markets
        .iter()
        .permutations(2)
        .map(|vec| {
            let source = *vec.get(0).expect("should be here");
            let destination = *vec.get(1).expect("here as well");
            (source, destination)
        })
        .map(|(source, destination)| (source, destination, get_profits(source, destination)))
        .map(|(source, destination, profit)| {
            (
                source,
                destination,
                profit,
                get_distance(source, destination),
            )
        })
        .map(|(source, destination, profit, distance)| {
            (
                source,
                destination,
                profit,
                distance,
                get_profit_per_distance(profit, distance),
            )
        })
        // .map(|(source, destination, profit, distance, profit_per_distance)|{
        //     println!("route: {}->{} profit: {}, distance: {}, profit per distance: {}", source.id, destination.id, profit, distance, profit_per_distance);
        //     (source, destination, profit, distance, profit_per_distance)
        // })
        .sorted_by(
            |(_, _, _, _, left_profit_per_distance), (_, _, _, _, right_profit_per_distance)| {
                right_profit_per_distance
                    .partial_cmp(left_profit_per_distance)
                    .unwrap_or_else(|| {
                        panic!(
                            "Couldn't order {} and {}",
                            left_profit_per_distance, right_profit_per_distance
                        )
                    })
            },
        )
        .next()
        .map(
            |(source, destination, profit, distance, profit_per_distance)| {
                (
                    source.id,
                    source.position.clone(),
                    destination.id,
                    profit,
                    distance,
                    profit_per_distance,
                )
            },
        )
        .expect("no routes?");

    Route {
        source: (source, source_position),
        destination,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let source = MarketWithPosition {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            position: Point2::new(20., 20.),
            food_buy_price: 2,
            food_sell_price: 7,
        };
        let destination = MarketWithPosition {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
            position: Point2::new(10., 10.),
            food_buy_price: 10,
            food_sell_price: 12,
        };
        let crappy_place = MarketWithPosition {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            position: Point2::new(20., 21.),
            food_buy_price: 2,
            food_sell_price: 20,
        };
        let good_but_to_far_away = MarketWithPosition {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap(),
            position: Point2::new(200000., 20.),
            food_buy_price: 1,
            food_sell_price: 2,
        };
        let markets = vec![
            destination.clone(),
            source.clone(),
            crappy_place,
            good_but_to_far_away,
        ];

        let result = get_most_profitable_route(&markets, &Point2::new(15., 15.));

        assert_eq!(result.source.0, source.id, "wrong source");
        assert_eq!(result.destination, destination.id, "wrong destination");
    }
}
