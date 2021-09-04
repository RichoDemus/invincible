use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::v2::market::Market;
use crate::v2::store::Store;
use crate::common_components::Name;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(planet_setup.system());
    }
}

pub struct Planet;

fn planet_setup(mut commands: Commands) {
    let planets = vec![
        ("Terra", false, Vec2::new(100.,100.), Color::CYAN, 20.),
        ("Agri", true, Vec2::new(-100.,-100.), Color::LIME_GREEN, 10.),
    ];
    for (name, magical_food, position, color, radius) in planets {
        let shape = shapes::Circle {
            radius,
            center: Vec2::default(),
        };

        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(color, Color::WHITE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform::default(),
        ))
            .insert(Transform::from_translation(position.extend(0.)))
            .insert(Name(name.to_string()))
            .with_children(|parent| {
                parent.spawn().insert(Store {
                    magically_produces_food: magical_food,
                    ..Store::default()
                });
            });
    }
}
