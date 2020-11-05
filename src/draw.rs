use legion::{component, IntoQuery, World};
use quicksilver::geom::{Circle, Rectangle, Vector};
use quicksilver::graphics::{Color, FontRenderer};
use quicksilver::Graphics;

use crate::components::Resource::Water;
use crate::components::{
    Name, NaturalResources, Planet, Population, Position, Selected, Shape, Stockpiles,
};
use crate::ship_components::Ship;

lazy_static! {
    static ref NAME_OFFSET: Vector = Vector::new(-20., 60.);
}

pub(crate) fn draw(gfx: &mut Graphics, world: &World, zoom_scale: f32, font: &mut FontRenderer) {
    draw_planets_with_natural_resources(gfx, world, zoom_scale);
    draw_planets_without_natural_resources(gfx, world, zoom_scale, font);
    draw_ships(gfx, world, zoom_scale);
    draw_selected_markers(gfx, world, zoom_scale);
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
                if natural_resources.resource == Water {
                    Color::BLUE
                } else {
                    Color::WHITE
                },
            );
        },
    );
}

fn draw_planets_without_natural_resources(
    gfx: &mut Graphics,
    world: &World,
    zoom_scale: f32,
    font: &mut FontRenderer,
) {
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

            <(&Position, &Stockpiles, &Population)>::query()
                .filter(component::<Selected>())
                .for_each(
                    world,
                    |(position, stockpiles, population): (&Position, &Stockpiles, &Population)| {
                        let position = Vector::new(
                            (position.point.x - 20.) as f32,
                            (position.point.y - 20.) as f32,
                        );

                        font.draw(
                            gfx,
                            // format!("FPS: {}", last_fps).as_str(),
                            // name.name.as_str(),
                            format!(
                                "Pop: {}\nStockpiles: {:?}",
                                population.population, stockpiles
                            )
                            .as_str(),
                            Color::GREEN,
                            position + *NAME_OFFSET,
                        )
                        .expect("failed to draw stockpiles");
                    },
                );
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
