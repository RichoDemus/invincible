use std::ops::Not;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::asset_loading::Fonts;
use crate::common_components::Name;
use crate::planet::Planet;
use crate::unit_selection::Selectable;
use crate::v2::commodity::Commodity;
use crate::v2::market::Market;
use crate::v2::store::Store;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ship_setup);
        app.add_system(ship_decision_system);
        app.add_system(move_ship_towards_objective);
        app.add_system(trade_with_planet);
    }
}

#[derive(Component)]
struct Ship;

#[derive(Default, Component)]
struct ActionQueue {
    queue: Vec<ShipAction>,
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
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                // todo, triangle instead of circle
                radius: 5.,
                center: Vec2::default(),
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GOLD),
                outline_mode: StrokeMode::new(Color::WHITE, 1.0),
            },
            Transform::default(),
        ))
        .insert(Ship)
        .insert(ActionQueue::default())
        .insert(Selectable::default())
        .with_children(|parent| {
            parent.spawn().insert_bundle(Text2dBundle {
                text: Text::with_section(
                    "Ship",
                    TextStyle {
                        font: fonts.font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform::from_xyz(0., -15., 0.),
                ..Default::default()
            });
        });
}

fn ship_decision_system(
    mut action_queues: Query<&mut ActionQueue>,
    stores: Query<(Entity, &Parent, &Store)>,
    names: Query<&Name>,
) {
    let food = Commodity::Food;
    for mut action_queue in action_queues.iter_mut() {
        if action_queue.queue.is_empty().not() {
            continue;
        }

        let (seller, seller_planet, sell_price) = stores
            .iter()
            .filter_map(|(entity, parent, store)| {
                store
                    .price_check_buy_from_store(&food)
                    .map(|credits| (entity, parent, credits))
            })
            .min_by_key(|(_, _, price)| *price)
            .expect("Should be a store to buy from");

        let (buyer, buyer_planet, buy_price) = stores
            .iter()
            .filter_map(|(entity, parent, store)| {
                store
                    .price_check_sell_to_store(&food)
                    .map(|credits| (entity, parent, credits))
            })
            .max_by_key(|(_, _, price)| *price)
            .expect("Should be a store to sell to");

        action_queue.queue.push(ShipAction::Buy {
            planet_to_buy_at: seller_planet.0,
            store: seller,
            commodity: food,
        });
        action_queue.queue.push(ShipAction::Sell {
            planet_to_sell_at: buyer_planet.0,
            store: buyer,
            commodity: food,
        });

        let seller_name = names
            .get_component::<Name>(seller_planet.0)
            .expect("No name for seller");
        let buyer_name = names
            .get_component::<Name>(buyer_planet.0)
            .expect("No name for seller");

        info!(
            "New trade, buy {:?} at {:?} for {}, sell att {:?} for {}",
            food, seller_name, sell_price, buyer_name, buy_price
        );
    }
}

fn move_ship_towards_objective(
    mut ships: Query<(&mut Transform, &mut ActionQueue), With<Ship>>,
    planets: Query<(Entity, &Transform), Without<Ship>>,
    time: Res<Time>,
) {
    for (mut ship_transform, mut action_queue) in ships.iter_mut() {
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

        ship_transform.translation += diff * time.delta_seconds() * 300.;
    }
}

fn trade_with_planet(
    mut ships: Query<(&mut Transform, &mut ActionQueue), With<Ship>>,
    planets: Query<(Entity, &Transform), Without<Ship>>,
    mut stores: Query<(Entity, &mut Store), Without<Ship>>,
) {
    let food = Commodity::Food;
    for (mut ship_transform, mut action_queue) in ships.iter_mut() {
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
                let mut store = stores
                    .get_component_mut::<Store>(*store)
                    .expect("Should be a store here");
                let receipt = store
                    .buy_from_store(food, 1, None)
                    .expect("should've managed a buy");
                action_queue.queue.remove(0);
                info!("Bought {:?} for {}", receipt.commodity, receipt.price);
            }
            ShipAction::Sell {
                store, commodity, ..
            } => {
                let mut store = stores
                    .get_component_mut::<Store>(*store)
                    .expect("Should be a store here");
                let receipt = store
                    .sell_to_store(food, 1, None)
                    .expect("should've managed a sell");
                action_queue.queue.remove(0);
                info!("Sold {:?} for {}", receipt.commodity, receipt.price);
            }
        }
    }
}
