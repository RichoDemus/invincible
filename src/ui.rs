use bevy::prelude::*;

use crate::asset_loading::Fonts;
use crate::unit_selection;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            unit_selection::add_selected_unit_info_panel(
                                parent,
                                fonts.font.clone(),
                            );
                        });
                });
        });
}
