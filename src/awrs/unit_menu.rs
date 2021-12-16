use bevy::prelude::*;

use super::{
    game::{AppState, GameState},
    unit::{Selected, Unit},
};

pub struct UnitMenu;

pub fn handle_open_unit_menu(
    mut commands: Commands,
    units_query: Query<&Unit, With<Selected>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("Opening unit menu...");

    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    for unit in units_query.iter() {
        info!("Found a unit, spawning node...");
        info!("location: ({}, {})", unit.location.x, unit.location.y);

        // TODO get unit menu options from selected unit.
        // Move if hasn't moved yet. Attack if unit next to it.
        let options = vec!["M - Move", "T - Attack", "C - Cancel"];

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                for text in options.into_iter() {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    text,
                                    TextStyle {
                                        font: asset_server.load("fonts/aw2-gba.otf"),
                                        font_size: 20.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                }
            })
            .insert(UnitMenu);
    }
}

pub fn handle_navigate_unit_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        info!("Changing Game State to MoveUnit");
        game_state
            .set(AppState::InGame(GameState::MoveUnit))
            .expect("Should be able to enter MoveUnit gamestate")
    }
    if keyboard_input.just_pressed(KeyCode::T) {
        info!("Performing Attack");

        game_state
            .set(AppState::InGame(GameState::ChooseTarget))
            .expect("Should be able to return to browsing")
    }
}

pub fn handle_exit_unit_menu(
    mut commands: Commands,
    mut unit_menu_query: Query<Entity, With<UnitMenu>>,
) {
    info!("Exiting Unit Menu");
    let unit_menu_entity = unit_menu_query.single_mut().unwrap();
    commands.entity(unit_menu_entity).despawn_recursive();
}
