use legion::{component, IntoQuery, World};
use quicksilver::geom::{Circle, Rectangle, Vector};
use quicksilver::graphics::{Color, FontRenderer};
use quicksilver::Graphics;

use crate::components::Resource::Water;
use crate::components::{Name, NaturalResources, Planet, Position, Selected, Stockpiles};

pub(crate) fn draw(gfx: &mut Graphics, world: &World, zoom_scale: f32, font: &mut FontRenderer) {
    let name_offset = Vector::new(-20., 60.);

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
            let circle = Circle::new(position, 10 as f32 * zoom_scale);
            gfx.fill_circle(
                &circle,
                if natural_resources.resource == Water {
                    Color::BLUE
                } else {
                    Color::WHITE
                },
            );

            // font.draw(
            //     gfx,
            //     // format!("FPS: {}", last_fps).as_str(),
            //     name.name.as_str(),
            //     Color::GREEN,
            //     position + name_offset,
            // ).expect("failed to draw planet name");
        },
    );

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

            <(&Position, &Stockpiles)>::query()
                .filter(component::<Selected>())
                .for_each(world, |(position, stockpiles): (&Position, &Stockpiles)| {
                    let position = Vector::new(
                        (position.point.x - 20.) as f32,
                        (position.point.y - 20.) as f32,
                    );
                    let rectangle = Rectangle::new(position, Vector::new(40., 40.));
                    gfx.stroke_rect(&rectangle, Color::GREEN);

                    font.draw(
                        gfx,
                        // format!("FPS: {}", last_fps).as_str(),
                        // name.name.as_str(),
                        format!("Stockpiles: {:?}", stockpiles).as_str(),
                        Color::GREEN,
                        position + name_offset,
                    )
                    .expect("failed to draw stockpiles");
                });

            // font.draw(
            //     gfx,
            //     // format!("FPS: {}", last_fps).as_str(),
            //     name.name.as_str(),
            //     Color::GREEN,
            //     position + name_offset,
            // ).expect("failed to draw planet name");
        },
    );
}
