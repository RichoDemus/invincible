use std::ops::Not;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use strum::IntoEnumIterator;

use crate::asset_loading::Fonts;
use crate::common_components::Name;
use crate::pause::AppState;
use crate::planet::NaturalResource::HydrogenGasVents;
use crate::planet::Planet;
use crate::unit_selection::Selectable;
use crate::v2::commodity::Commodity;
use crate::v2::commodity::Commodity::{Food, Fuel, HydrogenTanks};
use crate::v2::inventory::{Amount, Inventory};
use crate::v2::store::{Store, StoreListing};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ship_setup);
        app.add_systems(
            Update,
            (
                ship_decision_system,
                move_ship_towards_objective,
                trade_with_planet,
            ),
        );
        // app.add_system_set(
        //     SystemSet::on_update(AppState::GameRunning)
        //         .with_system(ship_decision_system)
        //         .with_system(move_ship_towards_objective)
        //         .with_system(trade_with_planet),
        // );
    }
}

#[derive(Component)]
struct Ship;

#[derive(Default, Component)]
struct ActionQueue {
    queue: Vec<ShipAction>,
}

#[derive(Component)]
struct Engine {
    speed: f32,
}

#[derive(Debug)]
enum ShipAction {
    Buy {
        planet_to_buy_at: Entity,
        store: Entity,
        commodity: Commodity,
    },
    Sell {
        planet_to_sell_at: Entity,
        store: Entity,
        commodity: Commodity,
    },
}

fn ship_setup(mut commands: Commands, fonts: Res<Fonts>) {
    for (ship_name, speed, capacity) in vec![("Wayfarer", 300., 5), ("Envoy", 100., 20)] {
        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        // todo, triangle instead of circle
                        radius: 5.,
                        center: Vec2::default(),
                    }),
                    ..Default::default()
                },
                Fill::color(Color::GOLD),
                Stroke::new(Color::WHITE, 1.),
            ))
            .insert(Ship)
            .insert(ActionQueue::default())
            .insert(Selectable::default())
            .insert(Engine { speed })
            .insert(Name(ship_name.to_string()))
            .insert(Inventory::with_capacity(capacity))
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        ship_name,
                        TextStyle {
                            font: fonts.font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_xyz(0., -15., 0.),
                    ..Default::default()
                });
            });
    }
}

fn ship_decision_system(
    mut action_queues: Query<(&mut ActionQueue, &Name)>,
    stores: Query<(Entity, &Planet, &Store, &Name)>,
) {
    for (mut action_queue, name) in action_queues.iter_mut() {
        if action_queue.queue.is_empty().not() {
            continue;
        }

        let buy_from_stores_listings = stores
            .iter()
            .map(
                |(entity, planet, store, name): (Entity, &Planet, &Store, &Name)| {
                    let listings = store.price_check_buy_from_store();
                    (entity, planet, name, listings)
                },
            )
            .collect::<Vec<_>>();

        let sell_to_stores_listings = stores
            .iter()
            .map(
                |(entity, planet, store, name): (Entity, &Planet, &Store, &Name)| {
                    let listings = store.price_check_sell_to_store();
                    (entity, planet, name, listings)
                },
            )
            .collect::<Vec<_>>();

        if let Some(trade_route) =
            decide_trade_route(buy_from_stores_listings, sell_to_stores_listings)
        {
            action_queue.queue.push(ShipAction::Buy {
                planet_to_buy_at: trade_route.store_to_buy_from,
                store: trade_route.store_to_buy_from,
                commodity: trade_route.commodity,
            });
            action_queue.queue.push(ShipAction::Sell {
                planet_to_sell_at: trade_route.store_to_sell_to,
                store: trade_route.store_to_sell_to,
                commodity: trade_route.commodity,
            });

            info!(
                "[{}]: New trade, buy {:?} at {:?} for {}, sell att {:?} for {}",
                name.0,
                trade_route.commodity,
                trade_route.store_to_buy_from_name,
                trade_route.cost_to_buy_commodity,
                trade_route.store_to_sell_to_name,
                trade_route.price_to_sell_commodity
            );
        } else {
            info!("No profitable trade possible");
        }
    }
}

fn move_ship_towards_objective(
    mut ships: Query<(&mut Transform, &mut ActionQueue, &Engine), With<Ship>>,
    planets: Query<(Entity, &Transform), Without<Ship>>,
    time: Res<Time>,
) {
    for (mut ship_transform, mut action_queue, engine) in ships.iter_mut() {
        if action_queue.queue.is_empty() {
            continue;
        }

        let action = action_queue.queue.first().expect("There's a action here");

        let destination_entity = match action {
            ShipAction::Buy {
                planet_to_buy_at: seller,
                ..
            } => seller,
            ShipAction::Sell {
                planet_to_sell_at: buyer,
                ..
            } => buyer,
        };

        let destination_transform = planets
            .get_component::<Transform>(*destination_entity)
            .expect("Planet has an entity");

        if destination_transform
            .translation
            .distance(ship_transform.translation)
            < 20.
        {
            continue;
        }

        // move towards destination
        let diff = destination_transform.translation - ship_transform.translation;
        let diff = diff.normalize();

        ship_transform.translation += diff * time.delta_seconds() * engine.speed;
    }
}

fn trade_with_planet(
    mut ships: Query<(&mut Transform, &mut ActionQueue, &mut Inventory), With<Ship>>,
    planets: Query<(Entity, &Transform), Without<Ship>>,
    mut stores: Query<(Entity, &mut Store), Without<Ship>>,
) {
    for (mut ship_transform, mut action_queue, mut inventory) in ships.iter_mut() {
        if action_queue.queue.is_empty() {
            continue;
        }

        let action = action_queue.queue.first().expect("There's a action here");

        let destination_entity = match action {
            ShipAction::Buy {
                planet_to_buy_at: seller,
                ..
            } => seller,
            ShipAction::Sell {
                planet_to_sell_at: buyer,
                ..
            } => buyer,
        };

        let destination_transform = planets
            .get_component::<Transform>(*destination_entity)
            .expect("Planet has an entity");

        if destination_transform
            .translation
            .distance(ship_transform.translation)
            > 20.
        {
            continue;
        }

        // we're at the right planet
        match action {
            ShipAction::Buy {
                store, commodity, ..
            } => {
                let amount_wanted = inventory.space_left();
                let mut store = stores
                    .get_component_mut::<Store>(*store)
                    .expect("Should be a store here");
                // todo maybe buy should handle this
                let amount_available = store.inventory.get(&commodity);
                let receipt = store
                    .buy_from_store(*commodity, amount_wanted.min(amount_available), None)
                    .expect("should've managed a buy");
                action_queue.queue.remove(0);
                inventory.add(receipt.commodity, receipt.amount);
                debug!("Bought {:?} for {}", receipt.commodity, receipt.price);
            }
            ShipAction::Sell {
                store, commodity, ..
            } => {
                let amount_to_sell = inventory.get(commodity);
                let mut store = stores
                    .get_component_mut::<Store>(*store)
                    .expect("Should be a store here");
                if let Some(receipt) = store.sell_to_store(*commodity, amount_to_sell, None) {
                    action_queue.queue.remove(0);
                    inventory.take(&receipt.commodity, receipt.amount);
                    debug!("Sold {:?} for {}", receipt.commodity, receipt.price);
                } else {
                    info!("Failed to sell {:?}, jettisoning it into space", commodity);
                    inventory.discard(*commodity);
                    action_queue.queue.remove(0);
                }
            }
        }
    }
}

struct TradeRoute {
    store_to_buy_from: Entity,
    store_to_buy_from_name: String,
    store_to_sell_to: Entity,
    store_to_sell_to_name: String,
    commodity: Commodity,
    cost_to_buy_commodity: Amount,
    price_to_sell_commodity: Amount,
}

fn decide_trade_route(
    buy_from_stores_listings: Vec<(Entity, &Planet, &Name, Vec<StoreListing>)>,
    sell_to_stores_listings: Vec<(Entity, &Planet, &Name, Vec<StoreListing>)>,
) -> Option<TradeRoute> {
    let buy_listings = buy_from_stores_listings
        .into_iter()
        .flat_map(|(entity, planet, name, listings)| {
            listings
                .into_iter()
                .map(|listing| (entity, planet, name, listing))
                .filter(|(_, _, _, listing)| listing.amount > 40) // todo make smarter
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let sell_listings = sell_to_stores_listings
        .into_iter()
        .flat_map(|(entity, planet, name, listings)| {
            listings
                .into_iter()
                .map(|listing| (entity, planet, name, listing))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // info!("Buy from store listings: {:#?}", buy_listings);
    // info!("Sell to store listings: {:#?}", sell_listings);
    let trade_routes = Commodity::iter()
        .into_iter()
        .flat_map(|commodity| {
            let cheapest_buy = buy_listings
                .iter()
                .filter(|(_, _, _, listing)| listing.commodity == commodity)
                .min_by_key(|(_, _, _, listing)| listing.price);
            let priciest_sell = sell_listings
                .iter()
                .filter(|(_, _, _, listing)| listing.commodity == commodity)
                .max_by_key(|(_, _, _, listing)| listing.price);
            // info!("trade: {:?} -> {:?}", cheapest_buy, priciest_sell);
            match (cheapest_buy, priciest_sell) {
                (
                    Some((buy_entity, _, buy_name, buy_listing)),
                    Some((sell_entity, _, sell_name, sell_listing)),
                ) if buy_listing.price < sell_listing.price => Some(TradeRoute {
                    store_to_buy_from: *buy_entity,
                    store_to_buy_from_name: buy_name.0.clone(),
                    store_to_sell_to: *sell_entity,
                    store_to_sell_to_name: sell_name.0.clone(),
                    commodity,
                    cost_to_buy_commodity: buy_listing.price,
                    price_to_sell_commodity: sell_listing.price,
                }),
                _ => None,
            }
        })
        .max_by_key(|trade_route| {
            trade_route.price_to_sell_commodity - trade_route.cost_to_buy_commodity
        });
    trade_routes
}
