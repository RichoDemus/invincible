use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (camera_system, camera_zoom_system));
    }
}

#[derive(Component)]
pub(crate) struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn camera_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in cameras.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 300. * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 300. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 300. * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 300. * time.delta_seconds();
        }
    }
}

// todo zoom towards mouse cursor
fn camera_zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut projection_query: Query<&mut Transform, With<MainCamera>>,
) {
    for event in mouse_wheel_events.iter() {
        let event: &MouseWheel = event;
        for mut transform in projection_query.iter_mut() {
            #[cfg(target_arch = "wasm32")]
            let zoom_amount = event.y * -0.001;
            #[cfg(not(target_arch = "wasm32"))]
            let zoom_amount = event.y * -0.1;
            let offset = Vec3::new(zoom_amount, zoom_amount, 0.);
            transform.scale += offset;
        }
    }
}

/// a bit stupid, assumes primary monitor
pub(crate) fn get_camera_position_in_world_coordinates(
    camera_query: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    Some(Vec2::new(0., 0.))
}

// /// a bit stupid, assumes primary monitor
// pub(crate) fn get_camera_position_in_world_coordinates(
//     windows: &Res<Windows>,
//     camera_query: &Query<&GlobalTransform, With<MainCamera>>,
// ) -> Option<Vec2> {
//     if let Some(window) = windows.get_primary() {
//         if let Some(cursor_position) = window.cursor_position() {
//             let global_transform = camera_query.single();
//             let norm = Vec3::new(
//                 cursor_position.x - window.width() / 2.,
//                 cursor_position.y - window.height() / 2.,
//                 0.,
//             );
//
//             let pos = *global_transform * norm;
//             return Some(pos.truncate());
//         }
//     }
//     None
// }
