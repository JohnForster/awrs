use bevy::prelude::*;

use crate::awrs::cursor::CursorStyle;

use super::{
    _cell::Cell,
    cursor::ChangeCursorEvent,
    game::{AppState, GameState},
    unit::{Selected, Unit},
};

pub struct UnitMenu;

pub fn open_unit_menu(
    mut commands: Commands,
    units_query: Query<&Cell, (With<Unit>, With<Selected>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_change_cursor: EventWriter<ChangeCursorEvent>,
    asset_server: Res<AssetServer>,
) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
    info!("Opening unit menu...");

    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    for location in units_query.iter() {
        info!("Found a unit, spawning node...");
        info!("location: ({}, {})", location.x, location.y);

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

pub fn unit_menu_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<AppState>>,
    units_query: Query<Entity, (With<Selected>, With<Unit>)>,
    mut commands: Commands,
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
    if keyboard_input.just_pressed(KeyCode::C) {
        info!("Returning to Browse");
        let unit_entity = units_query
            .single()
            .expect("Unit Menu is open but there is no unit selected?!");
        commands.entity(unit_entity).remove::<Selected>();

        game_state
            .set(AppState::InGame(GameState::Browsing))
            .expect("Should be able to return to browsing")
    }
}

pub fn exit_unit_menu(mut commands: Commands, mut unit_menu_query: Query<Entity, With<UnitMenu>>) {
    info!("Exiting Unit Menu");
    let unit_menu_entity = unit_menu_query.single_mut().unwrap();
    commands.entity(unit_menu_entity).despawn_recursive();
}
