use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::asset_loading::Fonts;
use crate::common_components::Name;
use crate::planet::NaturalResource::{FertileSoil, Hydrogen};
use crate::unit_selection::Selectable;
use crate::util::OncePerSecond;
use crate::v2::commodity::Commodity;
use crate::v2::market::Market;
use crate::v2::store::{Credits, Store};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(planet_setup);
        app.add_system(population_buys_food);
        app.add_system(produce_commodities_from_natural_resources);
    }
}

#[derive(Component)]
pub struct Planet;

// todo should these be separate structs instead?
#[derive(Component)]
pub struct PlanetaryResources {
    resources: Vec<NaturalResource>,
}
pub enum NaturalResource {
    FertileSoil,
    Hydrogen,
}

fn planet_setup(mut commands: Commands, fonts: Res<Fonts>) {
    let planets = vec![
        ("Terra", Vec2::new(100., 100.), Color::CYAN, 20., vec![]),
        (
            "Agri",
            Vec2::new(-100., -100.),
            Color::LIME_GREEN,
            10.,
            vec![FertileSoil],
        ),
        (
            "Hydro",
            Vec2::new(100., -100.),
            Color::PINK,
            30.,
            vec![Hydrogen],
        ),
        ("Forge", Vec2::new(-100., 100.), Color::GRAY, 15., vec![]),
    ];
    for (name, position, color, radius, natural_resources) in planets {
        let shape = shapes::Circle {
            radius,
            center: Vec2::default(),
        };

        let mut planet = commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(color),
                outline_mode: StrokeMode::new(Color::WHITE, 1.0),
            },
            Transform::default(),
        ));
        planet
            .insert(Planet)
            .insert(Transform::from_translation(position.extend(0.)))
            .insert(Name(name.to_string()))
            .insert(Selectable::default())
            .insert(PlanetaryResources {
                resources: natural_resources,
            })
            .insert(Store::default())
            .with_children(|parent| {
                parent.spawn().insert_bundle(Text2dBundle {
                    text: Text::with_section(
                        name,
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
                    transform: Transform::from_xyz(0., -25., 0.),
                    ..Default::default()
                });
            });
    }
}

// right now assumes 1 store per planet
fn population_buys_food(
    time: Res<Time>,
    mut once_per_second: Local<OncePerSecond>,
    mut stores: Query<&mut Store>,
) {
    if once_per_second.timer.tick(time.delta()).just_finished() {
        for mut store in stores.iter_mut() {
            match store.price_check_buy_from_store(&Commodity::Food) {
                Some(price) if price < 6 => {
                    // affordable food
                    let receipt = store
                        .buy_from_store(Commodity::Food, 1, Some(price))
                        .expect("We just checked, this should work");
                    info!("People bought food: {:?}", receipt);
                }
                _ => {
                    // no affordable food :o
                }
            }
        }
    }
}

fn produce_commodities_from_natural_resources(
    time: Res<Time>,
    mut once_per_second: Local<OncePerSecond>,
    mut stores: Query<(&mut Store, &PlanetaryResources)>,
) {
    if once_per_second.timer.tick(time.delta()).just_finished() {
        for (mut store, natural_resources) in stores.iter_mut() {
            for resource in &natural_resources.resources {
                match resource {
                    FertileSoil => {
                        store.give(Commodity::Food, 10);
                    }
                    Hydrogen => {
                        store.give(Commodity::Hydrogen, 20);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_produce_food() {
        let mut world = World::default();
        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(produce_commodities_from_natural_resources);

        let time = Time::default();
        // mock time? :S
        world.insert_resource(time);

        let dry_planet_store_entity = world.spawn().insert(Store::default()).id();
        let water_planet_store_entity = world.spawn().insert(Store::default()).id();

        let _water_world = world.spawn().push_children(&[dry_planet_store_entity]).id();

        let _non_water_world = world
            .spawn()
            .insert(Water)
            .push_children(&[water_planet_store_entity])
            .id();

        let dry_store = world.get::<Store>(dry_planet_store_entity).unwrap();
        let water_store = world.get::<Store>(water_planet_store_entity).unwrap();

        assert_eq!(dry_store.inventory.get(&Commodity::Food), 0);
        assert_eq!(water_store.inventory.get(&Commodity::Food), 0);

        update_stage.run(&mut world);

        let dry_store = world.get::<Store>(dry_planet_store_entity).unwrap();
        let water_store = world.get::<Store>(water_planet_store_entity).unwrap();

        assert_eq!(dry_store.inventory.get(&Commodity::Food), 0);
        assert_eq!(water_store.inventory.get(&Commodity::Food), 0);
    }
}
