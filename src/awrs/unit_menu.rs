use bevy::prelude::*;

use super::unit::Unit;

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
            let options = vec!["Move", "Attack", "Cancel"];

            commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::ColumnReverse,
                                align_content: AlignContent::FlexStart,
                                size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                                border: Rect::all(Val::Px(2.0)),

                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            for text in options.into_iter() {
                                build_button(&asset_server, parent, &mut materials, text);
                            }
                        });
                })
                .insert(UnitMenu)
                .insert(SelectedOption(0));
        }
    }
}

fn build_button(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    text: &str,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/aw2-gba.otf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

// Temporary to test out ui values
pub fn handle_unit_menu_navigation(
    keyboard_input: Res<Input<KeyCode>>,
    // button_materials: Res<ButtonMaterials>,
    mut ui_query: Query<(&mut Style, &mut SelectedOption), With<UnitMenu>>,
) {
    let (mut style, mut selected_option) =
        ui_query.single_mut().expect("Should have found one menu.");

    // if keyboard_input.just_pressed(KeyCode::W) {
    //     transform.translation.y += 1.0 * TILE_SIZE;
    //     cell.y += 1;
    // }

    // if keyboard_input.just_pressed(KeyCode::A) {
    //     transform.translation.x -= 1.0 * TILE_SIZE;
    //     cell.x -= 1;
    // }

    // if keyboard_input.just_pressed(KeyCode::S) {
    //     transform.translation.y -= 1.0 * TILE_SIZE;
    //     cell.y -= 1;
    // }

    // if keyboard_input.just_pressed(KeyCode::D) {
    //     transform.translation.x += 1.0 * TILE_SIZE;
    //     cell.x += 1;
    // }
}
