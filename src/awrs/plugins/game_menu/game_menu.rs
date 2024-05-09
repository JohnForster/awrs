use bevy::ecs::event::*;
use bevy::prelude::*;

use crate::awrs::{
    register_inputs::InputEvent,
    resources::{
        action_event::{Action, ActionEvent, ActionResultEvent},
        cursor::{ChangeCursorEvent, CursorStyle},
        map::ActiveTeam,
        state::{GameState, MenuState},
        unit::UnitId,
    },
};

#[derive(Component)]
pub struct GameMenu;

pub fn open_game_menu(
    mut commands: Commands,
    mut ev_change_cursor: EventWriter<ChangeCursorEvent>,
    asset_server: Res<AssetServer>,
) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
    info!("Opening game menu...");

    // ! Spawning ui camera on every time the menu is opened?

    // TODO get unit menu options from selected unit.
    // eg. Move if hasn't moved yet. Attack if unit next to it etc.
    let options = vec!["E - End Turn", "Enter - Return to game"];

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                ..Default::default()
            },
            GameMenu,
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

pub fn game_menu_input(
    mut input_events: ResMut<Events<InputEvent>>,
    mut ev_action: EventWriter<ActionEvent>,
    mut next_state: ResMut<NextState<MenuState>>,
) {
    let mut reader = input_events.get_reader();
    let mut should_clear = false;
    for ev in reader.read(&input_events) {
        match ev {
            InputEvent::EndTurn => {
                info!("Ending Turn");
                ev_action.send(ActionEvent(Action::EndTurn));
            }
            InputEvent::ToggleMenu => {
                info!("Quitting menu");

                next_state.set(MenuState::Closed);
                should_clear = true;
            }
            _ => {}
        };
    }
    if should_clear {
        input_events.clear();
    }
}

pub fn end_turn_result(
    mut ev_action_result: EventReader<ActionResultEvent>,
    mut q_units: Query<&mut Sprite, With<UnitId>>,
    mut active_team: ResMut<ActiveTeam>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    for action_result in ev_action_result.read() {
        if let ActionResultEvent::EndTurnResult(new_active_team) = action_result {
            info!("Handling end turn action result");
            active_team.team = *new_active_team;

            for mut sprite in q_units.iter_mut() {
                sprite.color = Color::WHITE;
            }

            next_game_state.set(GameState::Browsing);
            next_menu_state.set(MenuState::Closed);
        }
    }
}

pub fn exit_game_menu(mut commands: Commands, mut game_menu_query: Query<Entity, With<GameMenu>>) {
    info!("Exiting Game Menu");
    let game_menu_entity = game_menu_query.single_mut();
    commands.entity(game_menu_entity).despawn_recursive();
}
