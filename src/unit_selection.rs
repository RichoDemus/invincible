use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use strum::IntoEnumIterator;

use crate::asset_loading::Sprites;
use crate::camera::MainCamera;
use crate::common_components::Name;
use crate::v2::commodity::Commodity;
use crate::v2::inventory::Inventory;
use crate::v2::store::Store;

#[derive(Component)]
struct SelectedEntityInfoPanel;

pub(crate) struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (click_to_select_system, update_info_panel_system));
    }
}

#[derive(Default, Component)]
pub(crate) struct Selectable {
    pub(crate) selected: bool,
}

#[derive(Component)]
struct SelectionBox;

fn click_to_select_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    selectables: Query<(Entity, &Transform), With<Selectable>>,
    mut write_query: Query<(Entity, &mut Selectable)>,
    selection_boxes: Query<(Entity, &SelectionBox)>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some((camera, camera_transform)) = Some(camera_query.single()) {
            let mut clicked_entity = None;
            for (entity, transform) in selectables.iter() {
                let world_pos = camera
                    .viewport_to_world_2d(
                        camera_transform,
                        windows.single().cursor_position().unwrap(),
                    )
                    .unwrap(); // todo less unwraps
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
                            .spawn(SpriteBundle {
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
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(400.),
                border: UiRect::all(Val::Px(2.0)),
                // flex_wrap: FlexWrap::Wrap,
                // flex_direction: FlexDirection::Row,
                // justify_content: JustifyContent::Center,
                // align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::WHITE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        // position_type: PositionType::Absolute,
                        // position: Rect {
                        //     top: Val::Px(5.0),
                        //     left: Val::Px(15.0),
                        //     ..Default::default()
                        // },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font,
                            font_size: 16.,
                            color: Color::BLACK,
                            ..Default::default()
                        },
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
    if let Some((_selectable, name, maybe_store, maybe_inventory)) = selected_entity_query
        .iter()
        .find(|(selectable, _name, _, _)| selectable.selected)
    {
        if let Some(mut text) = info_box_query.iter_mut().next() {
            let text = text.sections.get_mut(0).unwrap();
            text.value = format!("Selected {}", name);

            if let Some(store) = maybe_store {
                text.value.push_str("\nX Name B/S");
                for commodity in Commodity::iter() {
                    let amount = store.inventory.get(&commodity);
                    let buy_price = store
                        .price_check_buy_specific_from_store(commodity)
                        .map(|l| l.price)
                        .unwrap_or(0);
                    let sell_price = store
                        .price_check_sell_specific_to_store(commodity)
                        .map(|l| l.price)
                        .unwrap_or(0);
                    text.value.push_str(&format!(
                        "\n{} {} {}/{}",
                        amount, commodity, buy_price, sell_price,
                    ));
                }
                // text.value.push_str(&format!(
                //     "\nFood: {}",
                //     store.inventory.get(&Commodity::Food)
                // ));
                // text.value.push_str(&format!(
                //     "\nHydrogen: {}",
                //     store.inventory.get(&Commodity::HydrogenTanks)
                // ));
                // text.value.push_str(&format!(
                //     "\nFuel: {}",
                //     store.inventory.get(&Commodity::Fuel)
                // ));
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
