use crate::{despawn_entities_with_component, GameState};
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Component)]
struct OnMainMenu;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(
            OnExit(GameState::Menu),
            despawn_entities_with_component::<OnMainMenu>,
        );
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up menu");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    border_color: Color::WHITE.into(),
                    background_color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Connect",
                        TextStyle {
                            font: asset_server.load("fonts/freefont/FreeSansBold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}
