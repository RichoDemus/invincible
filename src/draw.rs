use std::collections::HashMap;

use legion::{component, IntoQuery, World};
use quicksilver::geom::{Circle, Rectangle, Vector};
use quicksilver::graphics::{Color, FontRenderer};
use quicksilver::Graphics;
use uuid::Uuid;

use crate::components::Resource::Hydrogen;
use crate::components::Resource::Water;
use crate::components::{
    Id, Name, NaturalResources, Planet, Population, Position, Selected, Shape, Stockpiles,
};
use crate::economy_components::Market;
use crate::ship_components::{Ship, ShipObjective};

lazy_static! {
    static ref NAME_OFFSET: Vector = Vector::new(-20., 60.);
}

pub fn draw(gfx: &mut Graphics, world: &World, zoom_scale: f32, font: &mut FontRenderer) {
    draw_planets_with_natural_resources(gfx, world, zoom_scale);
    draw_planets_without_natural_resources(gfx, world, zoom_scale);
    draw_ships(gfx, world, zoom_scale);
    draw_selected_markers(gfx, world, zoom_scale);
    draw_selected_planet_info(gfx, world, font);
    draw_selected_ship_info(gfx, world, font);
}

fn draw_planets_with_natural_resources(gfx: &mut Graphics, world: &World, zoom_scale: f32) {
    let mut query = <(&Position, &Planet, &Name, &NaturalResources)>::query();
    query.for_each(
        world,
        |(position, _planet, _name, natural_resources): (
            &Position,
            &Planet,
            &Name,
            &NaturalResources,
        )| {
            let position = Vector::new(
                position.point.x as f32 * zoom_scale,
                position.point.y as f32 * zoom_scale,
            );
            let circle = Circle::new(position, 10. * zoom_scale);
            gfx.fill_circle(
                &circle,
                match natural_resources.resource {
                    Water => Color::BLUE,
                    Hydrogen => Color::PURPLE,
                    other => panic!(format!("Unhandled color: {:?}", other)),
                },
            );
        },
    );
}

fn draw_planets_without_natural_resources(gfx: &mut Graphics, world: &World, zoom_scale: f32) {
    let mut query = <(&Position, &Planet, &Name)>::query().filter(!component::<NaturalResources>());
    query.for_each(
        world,
        |(position, _planet, _name): (&Position, &Planet, &Name)| {
            let position = Vector::new(
                position.point.x as f32 * zoom_scale,
                position.point.y as f32 * zoom_scale,
            );
            let circle = Circle::new(position, 10. * zoom_scale);
            gfx.fill_circle(&circle, Color::WHITE);
        },
    );
}

fn draw_ships(gfx: &mut Graphics, world: &World, zoom_scale: f32) {
    <(&Ship, &Position, &Shape)>::query().for_each(
        world,
        |(_ship, position, shape): (&Ship, &Position, &Shape)| {
            let position = Vector::new(
                position.point.x as f32 * zoom_scale,
                position.point.y as f32 * zoom_scale,
            );
            let circle = Circle::new(position, shape.shape.radius as f32 * zoom_scale);
            gfx.fill_circle(&circle, Color::YELLOW);
        },
    );
}

fn draw_selected_markers(gfx: &mut Graphics, world: &World, _zoom_scale: f32) {
    <&Position>::query()
        .filter(component::<Selected>())
        .for_each(world, |position: &Position| {
            let position = Vector::new(
                (position.point.x - 20.) as f32,
                (position.point.y - 20.) as f32,
            );
            let rectangle = Rectangle::new(position, Vector::new(40., 40.));
            gfx.stroke_rect(&rectangle, Color::GREEN);
        });
}

fn draw_selected_planet_info(gfx: &mut Graphics, world: &World, font: &mut FontRenderer) {
    <(&Position, &Stockpiles, &Population, &Market, &Name)>::query()
        .filter(component::<Selected>())
        .for_each(
            world,
            |(position, stockpiles, population, market, name): (
                &Position,
                &Stockpiles,
                &Population,
                &Market,
                &Name,
            )| {
                let position = Vector::new(
                    (position.point.x - 20.) as f32,
                    (position.point.y - 20.) as f32,
                );

                font.draw(
                    gfx,
                    // format!("FPS: {}", last_fps).as_str(),
                    // name.name.as_str(),
                    format!(
                        "{}\nPop: {}\nStockpiles: {:?}\nFood (B/S): {}/{}",
                        name.name,
                        population.population,
                        stockpiles,
                        market.food_buy_price,
                        market.food_sell_price
                    )
                    .as_str(),
                    Color::GREEN,
                    position + *NAME_OFFSET,
                )
                .expect("failed to draw stockpiles");
            },
        );
}

fn draw_selected_ship_info(gfx: &mut Graphics, world: &World, font: &mut FontRenderer) {
    let id_name_lookup = <(&Id, &Name)>::query()
        .iter(world)
        .map(|(id, name): (&Id, &Name)| (id.uuid, name.name.clone()))
        .collect::<HashMap<Uuid, String>>();

    <(&Position, &Ship, &Stockpiles)>::query()
        .filter(component::<Selected>())
        .for_each(
            world,
            |(position, ship, stockpiles): (&Position, &Ship, &Stockpiles)| {
                let position = Vector::new(
                    (position.point.x - 20.) as f32,
                    (position.point.y - 20.) as f32,
                );

                let objective = match ship.objective {
                    ShipObjective::Idle => String::from("Idle"),
                    ShipObjective::TravelTo(destination) => format!(
                        "Travelling to {}",
                        id_name_lookup
                            .get(&destination)
                            .expect("No such destination")
                    ),
                    // ShipObjective::DockedAt(dock) => format!("Docked at {}", id_name_lookup.get(&dock).expect("No such destination")),
                };

                font.draw(
                    gfx,
                    format!(
                        "Objective: {:?}\nStockpiles: {:?}",
                        objective, stockpiles.stockpiles
                    )
                    .as_str(),
                    Color::GREEN,
                    position + *NAME_OFFSET,
                )
                .expect("failed to draw stockpiles");
            },
        );
}
