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
mod ship;
pub mod ui;
mod unit_selection;
pub mod util;
pub mod v2;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Invincible".to_string(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            AssetLoadingPlugin,
            ShipPlugin,
            PlanetPlugin,
            CameraPlugin,
            ShapePlugin,
            UiPlugin,
            SelectPlugin,
            PausePlugin,
        ))
        .run();
}
