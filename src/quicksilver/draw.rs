use std::collections::HashMap;

use quicksilver::geom::{Circle, Rectangle, Vector};
use quicksilver::graphics::{Color, FontRenderer};
use quicksilver::Graphics;
use uuid::Uuid;

use crate::quicksilver::{HEIGHT, WIDTH};
use crate::quicksilver::core::Core;
use crate::quicksilver::selectability::{SelectableAndPositionAndShape, Selectable};
use crate::quicksilver::ship::ShipObjective;
use crate::quicksilver::market_calculations::Commodity;

lazy_static! {
    static ref NAME_OFFSET: Vector = Vector::new(-20., 60.);
}

pub fn draw(gfx: &mut Graphics, zoom_scale: f32, font: &mut FontRenderer, core: &Core) {
    draw_planets(gfx, core, zoom_scale);
    draw_selected_markers(gfx, core, zoom_scale);
    draw_selected_planet_info(gfx, core, font);
    draw_ships(gfx, core, zoom_scale);
    draw_selected_ship_info(gfx, core, font);
}

fn draw_planets(gfx: &mut Graphics, core: &Core, zoom_scale: f32) {
    for (_, planet) in &core.planets {
        let position = Vector::new(
            planet.position.x as f32 * zoom_scale,
            planet.position.y as f32 * zoom_scale,
        );
        let circle = Circle::new(position, 10. * zoom_scale);
        gfx.fill_circle(
            &circle,
if planet.water {
    Color::BLUE
} else if planet.hydrogen {
    Color::MAGENTA
} else {
    Color::from_rgba(46, 49, 49, 1.)
},
        );
    }
}

fn draw_ships(gfx: &mut Graphics, core: &Core, zoom_scale: f32) {
    for (_, ship) in &core.ships {
        let position = Vector::new(
            ship.position.x as f32 * zoom_scale,
            ship.position.y as f32 * zoom_scale,
        );
        let circle = Circle::new(position, ship.shape.radius as f32 * zoom_scale);
        gfx.fill_circle(&circle, Color::YELLOW);
    }
}

fn draw_selected_markers(gfx: &mut Graphics, core: &Core, _zoom_scale: f32) {
    let ships = core.ships.values().map(|e|{
        let selectable: &dyn SelectableAndPositionAndShape = e;
        selectable
    });
    let planets = core.planets.values().map(|e|{
        let selectable: &dyn SelectableAndPositionAndShape = e;
        selectable
    });
    for selectable in ships.chain(planets) {
        if selectable.selected() {
            let position = Vector::new(
                (selectable.position_and_shape().0.x - 20.) as f32,
                (selectable.position_and_shape().0.y - 20.) as f32,
            );
            let rectangle = Rectangle::new(position, Vector::new(40., 40.));
            gfx.stroke_rect(&rectangle, Color::GREEN);
        }
    }
}

fn draw_selected_planet_info(gfx: &mut Graphics, core: &Core, font: &mut FontRenderer) {
    for planet in core.planets.values() {
        if planet.selected {
            let mut x = (planet.position.x - 20.) as f32;
            if x + 100. > WIDTH {
                x -= 150.
            }
            if x < 100. {
                x += 150.
            }
            let mut y = (planet.position.y - 20.) as f32;
            if y + 100. > HEIGHT {
                y -= 150.
            }
            let position = Vector::new(x, y);

            font.draw(
                gfx,
                // format!("FPS: {}", last_fps).as_str(),
                // name.name.as_str(),
                format!(
                    "{}\nPop: {}\nStockpiles: {:?}\nFood: {:?}\nSell: {:?}",
                    planet.name,
                    planet.population,
                    planet.items.items,
                    planet.market_orders,
                    "",
                )
                    .as_str(),
                Color::GREEN,
                position + *NAME_OFFSET,
            )
                .expect("failed to draw stockpiles");
        } else {
            let mut x = (planet.position.x - 20.) as f32;
            if x + 100. > WIDTH {
                x -= 150.
            }
            if x < 100. {
                x += 150.
            }
            let mut y = (planet.position.y - 20.) as f32;
            if y + 100. > HEIGHT {
                y -= 150.
            }
            let position = Vector::new(x, y);

            let food_amount = planet.items.items.get(&Commodity::Food).unwrap_or(&0);
            font.draw(
                gfx,
                // format!("FPS: {}", last_fps).as_str(),
                // name.name.as_str(),
                format!(
                    "{}. f: {}",//\nPop: {}\nStockpiles: {:?}\nFood: {:?}\nSell: {:?}",
                    planet.name,
                    food_amount,
                    // planet.population,
                    // planet.items.items,
                    // planet.market_orders,
                    // "",
                )
                    .as_str(),
                Color::GREEN,
                position + *NAME_OFFSET,
            )
                .expect("failed to draw stockpiles");
        }
    }
}

fn draw_selected_ship_info(gfx: &mut Graphics, core: &Core, font: &mut FontRenderer) {
    for ship in core.ships.values() {
        if ship.selected() {
            let position = Vector::new(
                (ship.position.x - 20.) as f32,
                (ship.position.y - 20.) as f32,
            );

            let objective = match &ship.objective {
                ShipObjective::Idle(str) => format!("Idle: {:?}", str),
                ShipObjective::TravelTo(destination) => format!(
                    "Travelling to {}",
                    core.planets
                        .get(&destination)
                        .expect("No such destination")
                        .name
                ),
                // ShipObjective::DockedAt(dock) => format!("Docked at {}", id_name_lookup.get(&dock).expect("No such destination")),
            };

            font.draw(
                gfx,
                format!(
                    "Objective: {:?}\nStockpiles: {:?}",
                    objective, ship.inventory.items
                )
                    .as_str(),
                Color::GREEN,
                position + *NAME_OFFSET,
            )
                .expect("failed to draw stockpiles");
        }
    }
}
