use bevy::prelude::*;

use crate::awrs::{
    engine::ScenarioState,
    register_inputs::InputEvent,
    resources::{
        cursor::{ChangeCursorEvent, CursorStyle},
        map::ActiveTeam,
        state::GameState,
        unit::{Selected, UnitId},
    },
};

pub struct GameMenu;

pub fn open_game_menu(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_change_cursor: EventWriter<ChangeCursorEvent>,
    asset_server: Res<AssetServer>,
) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
    info!("Opening game menu...");

    // ! Spawning ui camera on every time the menu is opened?
    commands.spawn_bundle(UiCameraBundle::default());

    // TODO get unit menu options from selected unit.
    // eg. Move if hasn't moved yet. Attack if unit next to it etc.
    let options = vec!["E - End Turn", "Enter - Return to game"];

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
        .insert(GameMenu);
}

pub fn game_menu_input(
    mut ev_input: EventReader<InputEvent>,
    mut game_state: ResMut<State<GameState>>,
    units_query: Query<Entity, (With<Selected>, With<UnitId>)>,
    scenario_state: Res<ScenarioState>,
    mut active_team: ResMut<ActiveTeam>,
    mut commands: Commands,
) {
    for ev in ev_input.iter() {
        match ev {
            InputEvent::EndTurn => {
                info!("Ending Turn");
                active_team.team = (active_team.team + 1) % scenario_state.teams.len() as u32;
                game_state
                    .set(GameState::Browsing)
                    .expect("Should be able to return to Browsing gamestate");
            }
            InputEvent::ToggleMenu => {
                info!("Quitting menu");

                game_state.pop();
            }
            _ => {}
        };
    }
}

pub fn exit_game_menu(mut commands: Commands, mut game_menu_query: Query<Entity, With<GameMenu>>) {
    info!("Exiting Game Menu");
    let game_menu_entity = game_menu_query.single_mut().unwrap();
    commands.entity(game_menu_entity).despawn_recursive();
}
