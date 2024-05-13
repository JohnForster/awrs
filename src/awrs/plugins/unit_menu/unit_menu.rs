use bevy::prelude::*;

use crate::awrs::resources::{
    cursor::{ChangeCursorEvent, CursorStyle},
    state::GameState,
    unit::{Selected, UnitId},
};

#[derive(Component)]
pub struct UnitMenu;

pub fn open_unit_menu(
    mut commands: Commands,
    units_query: Query<&UnitId, With<Selected>>,
    mut ev_change_cursor: EventWriter<ChangeCursorEvent>,
    asset_server: Res<AssetServer>,
) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
    info!("Opening unit menu...");

    for _id in units_query.iter() {
        info!("Found a unit, spawning node...");

        // TODO get unit menu options from selected unit.
        // eg. Move if hasn't moved yet. Attack if unit next to it etc.
        let options = vec!["M - Move", "T - Attack", "C - Cancel"];

        commands
            .spawn((
                UnitMenu,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                for text in options.into_iter() {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    text,
                                    TextStyle {
                                        font: asset_server.load("fonts/aw2-gba.otf"),
                                        font_size: 20.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                ..Default::default()
                            });
                        });
                }
            });
    }
}

pub fn unit_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    units_query: Query<Entity, (With<Selected>, With<UnitId>)>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        info!("Changing Game State to MoveUnit");
        next_state.set(GameState::MoveUnit);
    }
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        info!("Performing Attack");

        next_state.set(GameState::ChooseTarget);
    }
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        info!("Cancel selected. Returning to Browse");
        if let Ok(unit_entity) = units_query.get_single() {
            info!("Clearing selected unit");
            commands.entity(unit_entity).remove::<Selected>();
        }

        next_state.set(GameState::Browsing);
    }
}

pub fn exit_unit_menu(mut commands: Commands, mut unit_menu_query: Query<Entity, With<UnitMenu>>) {
    info!("Exiting Unit Menu");
    for unit_menu_entity in unit_menu_query.iter_mut() {
        commands.entity(unit_menu_entity).despawn_recursive();
    }
}
