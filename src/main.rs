#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::asset_loading::AssetLoadingPlugin;
use crate::camera::CameraPlugin;
use crate::pause::PausePlugin;
use crate::planet::PlanetPlugin;
use crate::ship::ShipPlugin;
use crate::ui::UiPlugin;
use crate::unit_selection::SelectPlugin;

mod asset_loading;
mod camera;
pub mod common_components;
mod pause;
mod planet;
mod quicksilver;
mod ship;
pub mod ui;
mod unit_selection;
pub mod util;
pub mod v2;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Invincible".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: false,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadingPlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(PlanetPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(SelectPlugin)
        .add_plugin(PausePlugin)
        .run();
}
