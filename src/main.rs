use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Running)
        .add_startup_system(load_terrain_sprites.system().label("load_sprites"))
        .add_startup_system(load_unit_sprites.system().label("load_sprites"))
        .add_startup_system(load_ui_sprites.system().label("load_sprites"))
        // Can use SystemSet::on_enter and SystemSet::on_exit to run setup and cleanup code.
        .add_system_set(
            SystemSet::on_enter(GameState::Running)
                .with_system(build_map.system())
                .with_system(create_cursor.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(handle_cursor_move.system())
                .with_system(handle_cursor_select.system()),
        )
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

#[derive(Default)]
struct TerrainAtlas {
    texture_handle: Handle<Texture>,
    atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
struct UIAtlas {
    texture_handle: Handle<Texture>,
    atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
struct UnitAtlas {
    texture_handle: Handle<Texture>,
    atlas_handle: Handle<TextureAtlas>,
}

fn load_ui_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading UI Sprites");
    let texture_handle = asset_server.load("UISprites.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(143.0, 64.0));

    let cursor_rect = bevy::sprite::Rect {
        min: Vec2::new(44.0, 6.0),
        max: Vec2::new(44.0 + 28.0, 6.0 + 32.0),
    };

    texture_atlas.add_texture(cursor_rect);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(UIAtlas {
        texture_handle,
        atlas_handle,
    })
}

fn load_unit_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading Unit Sprites");
    let texture_handle = asset_server.load("unitSprites.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(349.0, 93.0));

    let inf_orange_sprite = bevy::sprite::Rect {
        min: Vec2::new(24.0, 4.0),
        max: Vec2::new(24.0 + 14.0, 4.0 + 16.0),
    };

    texture_atlas.add_texture(inf_orange_sprite);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(UnitAtlas {
        texture_handle,
        atlas_handle,
    })
}

fn load_terrain_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading Terrain Sprites");
    // Terrain Sprites
    let texture_handle = asset_server.load("sprites.png");
    let mut texture_atlas =
        TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(1215.0, 1744.0));

    let grass_rect = bevy::sprite::Rect {
        min: Vec2::new(217.0, 1567.0),
        max: Vec2::new(217.0 + 16.0, 1567.0 + 16.0),
    };
    let sea_rect = bevy::sprite::Rect {
        min: Vec2::new(340.0, 1567.0),
        max: Vec2::new(340.0 + 16.0, 1567.0 + 16.0),
    };

    texture_atlas.add_texture(sea_rect);
    texture_atlas.add_texture(grass_rect);

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(TerrainAtlas {
        texture_handle,
        atlas_handle,
    })
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
