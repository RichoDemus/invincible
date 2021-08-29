#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;

mod quicksilver;
mod v2;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Invcincible".to_string(),
            width: 800.,
            height: 600.,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
