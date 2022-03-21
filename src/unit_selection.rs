use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;

use crate::asset_loading::Sprites;
use crate::camera::{get_camera_position_in_world_coordinates, MainCamera};
use crate::common_components::Name;
use crate::v2::commodity::Commodity;
use crate::v2::inventory::Inventory;
use crate::v2::store::Store;

#[derive(Component)]
struct SelectedEntityInfoPanel;

pub(crate) struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(load_sprite_system.system());
        app.add_system(click_to_select_system);
        app.add_system(update_info_panel_system);
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

pub(crate) fn add_selected_unit_info_panel(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(400.0)),
                border: Rect::all(Val::Px(2.0)),
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            color: Color::WHITE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(5.0),
                            left: Val::Px(15.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font,
                            font_size: 16.,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(SelectedEntityInfoPanel);
        });
}

fn update_info_panel_system(
    mut info_box_query: Query<&mut Text, With<SelectedEntityInfoPanel>>,
    selected_entity_query: Query<(&Selectable, &Name, Option<&Store>, Option<&Inventory>)>,
) {
    if let Some((_selectable, name, maybe_market, maybe_inventory)) = selected_entity_query
        .iter()
        .find(|(selectable, name, _, _)| selectable.selected)
    {
        if let Some(mut text) = info_box_query.iter_mut().next() {
            let text = text.sections.get_mut(0).unwrap();
            text.value = format!("Selected {}", name);

            if let Some(market) = maybe_market {
                text.value.push_str(&format!(
                    "\nFood: {}",
                    market.inventory.get(&Commodity::Food)
                ));
                text.value.push_str(&format!(
                    "\nHydrogen: {}",
                    market.inventory.get(&Commodity::HydrogenTanks)
                ));
                text.value.push_str(&format!(
                    "\nFuel: {}",
                    market.inventory.get(&Commodity::Fuel)
                ));
            }
            if let Some(inventory) = maybe_inventory {
                text.value
                    .push_str(&format!("\nFood: {}", inventory.get(&Commodity::Food)));
                text.value.push_str(&format!(
                    "\nHydrogen: {}",
                    inventory.get(&Commodity::HydrogenTanks)
                ));
                text.value
                    .push_str(&format!("\nFuel: {}", inventory.get(&Commodity::Fuel)));
            }
        }
    } else {
        if let Some(mut text) = info_box_query.iter_mut().next() {
            let text = text.sections.get_mut(0).unwrap();
            text.value = "Nothing selected".to_string();
        }
    }
}
