use bevy::prelude::*;

use super::{game::GameState, unit::Unit};

pub struct UnitMenu;
pub struct SelectedOption(usize);

pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub fn open_unit_menu(
    mut commands: Commands,
    units_query: Query<&Unit>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("Opening unit menu...");

    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    for unit in units_query.iter() {
        if unit.selected.0 {
            info!("Found a unit, spawning node...");
            info!("location: ({}, {})", unit.location.x, unit.location.y);

            // TODO get unit menu options from selected unit.
            // Move if hasn't moved yet. Attack if unit next to it.
            let options = vec!["M - Move", "A - Attack", "C - Cancel"];

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
                .insert(UnitMenu)
                .insert(SelectedOption(0));
        }
    }
}

pub fn handle_unit_menu_navigation(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        info!("Changing Game State to MoveUnit");
        game_state
            .set(GameState::MoveUnit)
            .expect("Should be able to enter MoveUnit gamestate")
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
