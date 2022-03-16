#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::asset_loading::AssetLoadingPlugin;
use crate::camera::CameraPlugin;
use crate::planet::PlanetPlugin;
use crate::ship::ShipPlugin;

mod asset_loading;
mod camera;
pub mod common_components;
mod planet;
mod quicksilver;
mod ship;
pub mod util;
pub mod v2;

fn main() {
    App::new()
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
