use bevy::prelude::*;

mod awrs;

use awrs::sprite_loading::*;



pub struct AWRSPlugin;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(load_terrain_sprites.system().label("load_sprites"))
            .add_startup_system(load_unit_sprites.system().label("load_sprites"))
            .add_startup_system(load_ui_sprites.system().label("load_sprites"))
            .add_state(GameState::Running) // see if this can go after the next two method calls
            .add_system_set(
                SystemSet::on_enter(GameState::Running)
                    .with_system(build_map.system())
                    .with_system(create_cursor.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_select.system()));
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AWRSPlugin)
        .run();
}

const TILE_SIZE: f32 = 16.0;

enum AppState {
    MainMenu,
    InGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Paused,
    Running,
    UnitMenu,
    BuildingMenu,
    MoveUnit,
    EnemyTurn,
}



#[derive(Clone)]
struct UnitHealth(u32);
#[derive(Clone)]
enum UnitType {
    Infantry,
}

#[derive(Clone)]
struct Cell {
    x: u32,
    y: u32,
}

#[derive(Clone)]
struct Team(u32);

#[derive(Clone)]
struct Unit {
    unit_type: UnitType,
    team: Team,
    location: Cell,
    health: UnitHealth, // etc. etc..
}

#[derive(Bundle)]
struct UnitBundle {
    id: usize,
    data: Unit,
    #[bundle]
    sprite: SpriteSheetBundle,
}

// TODO Load sprites from json: https://github.com/serde-rs/json
fn build_map(mut commands: Commands, terrain_atlas: Res<TerrainAtlas>, unit_atlas: Res<UnitAtlas>) {
    info!("Building Map");
    let game_map = vec![
        vec![0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 1, 1],
        vec![0, 0, 1, 1, 1, 1],
        vec![0, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1],
    ];

    let units = vec![Unit {
        unit_type: UnitType::Infantry,
        team: Team(0),
        location: Cell { x: 1, y: 1 },
        health: UnitHealth(100),
    }];

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for (y, row) in game_map.iter().rev().enumerate() {
        for (x, &terrain_index) in row.iter().enumerate() {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: terrain_atlas.atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(terrain_index),
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    0.0,
                )),
                ..Default::default()
            });
        }
    }

    for (i, unit) in units.iter().enumerate() {
        let x = unit.location.x;
        let y = unit.location.y;
        commands.spawn_bundle(UnitBundle {
            id: i,
            data: unit.clone(),
            sprite: SpriteSheetBundle {
                texture_atlas: unit_atlas.atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    1.0,
                )),
                ..Default::default()
            },
        });
    }
}

struct Cursor;

fn create_cursor(mut commands: Commands, ui_atlas: Res<UIAtlas>) {
    info!("Creating Cursor");
    let x = 0;
    let y = 0;
    let starting_position = Vec3::new(x as f32, y as f32, 0.0) * TILE_SIZE;
    let adjustment = Vec3::new(4.0, -5.0, 1.0);

    // Combine these into the Cursor struct?
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui_atlas.atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(starting_position + adjustment),
            ..Default::default()
        })
        .insert(Cursor)
        .insert(Cell { x, y })
        .insert(Timer::from_seconds(0.075, false));
}

fn handle_cursor_move(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_query: Query<(&mut Timer, &mut Transform, &mut Cell, &Cursor)>,
) {
    for (mut _timer, mut transform, mut cell, _) in cursor_query.iter_mut() {
        // timer.tick(time.delta());

        // if !timer.finished() {
        //     continue;
        // }

        if keyboard_input.just_pressed(KeyCode::W) {
            transform.translation.y += 1.0 * TILE_SIZE;
            cell.y += 1;
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            transform.translation.x -= 1.0 * TILE_SIZE;
            cell.x -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::S) {
            transform.translation.y -= 1.0 * TILE_SIZE;
            cell.y -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::D) {
            transform.translation.x += 1.0 * TILE_SIZE;
            cell.x += 1;
        }

        // timer.reset()
    }
}

fn handle_cursor_select(
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_query: Query<(&mut Transform, &Cell, &Cursor)>,
    mut units_query: Query<&Unit>,
    mut commands: Commands,
) {
    for (mut cursor_transform, cursor_cell, _) in cursor_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for unit in units_query.iter_mut() {
                let unit_cell = &unit.location;
                if unit_cell.x == cursor_cell.x && unit_cell.y == cursor_cell.y {
                    info!("Selected a unit!");
                }
            }
        }
    }
}
