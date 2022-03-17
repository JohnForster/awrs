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
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                color: Color::NONE.into(),
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
                            color: Color::NONE.into(),
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
    mut st_game: ResMut<State<GameState>>,
    units_query: Query<Entity, (With<Selected>, With<UnitId>)>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        info!("Changing Game State to MoveUnit");
        st_game
            .set(GameState::MoveUnit)
            .expect("Should be able to enter MoveUnit gamestate")
    }
    if keyboard_input.just_pressed(KeyCode::T) {
        info!("Performing Attack");

        st_game
            .set(GameState::ChooseTarget)
            .expect("Should be able to return to browsing")
    }
    if keyboard_input.just_pressed(KeyCode::C) {
        info!("Returning to Browse");
        let unit_entity = units_query.single();

        info!("Clearing selected unit");
        commands.entity(unit_entity).remove::<Selected>();

        st_game
            .set(GameState::Browsing)
            .expect("Should be able to return to browsing")
    }
}

pub fn exit_unit_menu(mut commands: Commands, mut unit_menu_query: Query<Entity, With<UnitMenu>>) {
    info!("Exiting Unit Menu");
    let unit_menu_entity = unit_menu_query.single_mut();
    commands.entity(unit_menu_entity).despawn_recursive();
}
