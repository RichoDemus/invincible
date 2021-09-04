#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::asset_loading::AssetLoadingPlugin;
use crate::ship::ShipPlugin;
use crate::camera::CameraPlugin;
use crate::planet::PlanetPlugin;

mod quicksilver;
pub mod v2;
mod asset_loading;
mod ship;
mod camera;
mod planet;
pub mod common_components;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Invincible".to_string(),
            width: 800.,
            height: 600.,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadingPlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(PlanetPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ShapePlugin)
        .run();
}
