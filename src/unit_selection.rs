use bevy::prelude::*;

use crate::asset_loading::Sprites;
use crate::camera::{get_camera_position_in_world_coordinates, MainCamera};

pub(crate) struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(load_sprite_system.system());
        app.add_system(click_to_select_system.system());
        // app.add_system(update_info_panel_system.system());
    }
}

#[derive(Default, Component)]
pub(crate) struct Selectable {
    pub(crate) selected: bool,
}

#[derive(Component)]
struct SelectionBox;

fn click_to_select_system(
    windows: Res<Windows>,
    selectables: Query<(Entity, &Transform), With<Selectable>>,
    mut write_query: Query<(Entity, &mut Selectable)>,
    selection_boxes: Query<(Entity, &SelectionBox)>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<&GlobalTransform, With<MainCamera>>,
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = get_camera_position_in_world_coordinates(&windows, &camera_query) {
            let mut clicked_entity = None;
            for (entity, transform) in selectables.iter() {
                if world_pos.distance(transform.translation.truncate()) < 50. {
                    clicked_entity = Some(entity);
                    break;
                }
            }

            // regardless of if we clicked something or not, deselect current thing
            for (entity, _) in selection_boxes.iter() {
                commands.entity(entity).despawn();
            }
            for (_, mut selectable) in write_query.iter_mut() {
                selectable.selected = false;
            }

            if let Some(clicked_entity) = clicked_entity {
                for (entity, mut selectable) in write_query.iter_mut() {
                    if clicked_entity == entity {
                        selectable.selected = true;
                        let selection_box = commands
                            .spawn_bundle(SpriteBundle {
                                texture: sprites.selection_box.clone(),
                                sprite: Sprite {
                                    color: Color::GREEN,
                                    ..Default::default()
                                },
                                // material: materials.add(sprites.selection_box.clone().into()),
                                // transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                                ..Default::default()
                            })
                            .insert(SelectionBox)
                            .id();

                        commands.entity(entity).push_children(&[selection_box]);
                    }
                }
            }
        }
    }
}
