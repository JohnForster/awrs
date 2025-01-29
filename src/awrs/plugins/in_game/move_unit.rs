use bevy::prelude::*;

use advance_craft_engine::Tile as EngineTile;

use crate::awrs::{
    register_inputs::InputEvent,
    resources::{
        action_event::{Action, ActionEvent},
        atlases::ArrowAtlas,
        cursor::{ChangeCursorEvent, CursorStyle},
        scenario::ScenarioState,
        state::GameState,
        tile::{Tile, TILE_SIZE},
        unit::{Selected, UnitId},
    },
};

pub struct MoveUnitPlugin;

impl Plugin for MoveUnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitPlan {
            range: 0,
            steps: vec![],
        })
        .add_event::<PlanUpdateEvent>()
        .add_event::<ConfirmMoveEvent>()
        .add_systems(OnEnter(GameState::MoveUnit), begin_unit_plan)
        .add_systems(
            Update,
            (update_arrows, update_movement_plan, confirm_move)
                .run_if(in_state(GameState::MoveUnit))
                .chain(),
        )
        .add_systems(OnExit(GameState::MoveUnit), exit_movement_plan);
    }
}

#[derive(Resource)]
pub struct UnitPlan {
    pub range: u32,
    pub steps: Vec<MoveStep>,
}

#[derive(Component)]
pub struct MoveStepSprite;

pub struct MoveStep {
    pub tile: Tile,
    pub entity: Entity,
}

type Index = usize;
enum PlanChange {
    Remove(Index),
    Add(Tile),
    Invalid,
}

#[derive(Event)]
pub struct ConfirmMoveEvent;

pub fn begin_unit_plan(
    mut q_selected_unit: Query<(&UnitId, &mut Transform), With<Selected>>,
    mut ev_change_cursor: EventWriter<ChangeCursorEvent>,
    scenario_state: Res<ScenarioState>,
    mut unit_plan: ResMut<UnitPlan>,
    mut commands: Commands,
    arrow_atlas: Res<ArrowAtlas>,
) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::None));

    let (UnitId(unit_id), unit_transform) = q_selected_unit.single_mut();

    let range = scenario_state.get_movement_range(&unit_id);
    unit_plan.range = range;
    unit_plan.steps = vec![];

    if let Some(tile) = check_valid(&unit_transform, &scenario_state, *unit_id, (0, 0)) {
        add_tile(tile, &mut unit_plan, &arrow_atlas, &mut commands)
    };
}

pub fn update_movement_plan(
    mut q_selected_unit: Query<(&UnitId, &mut Transform), With<Selected>>,
    scenario_state: Res<ScenarioState>,
    mut unit_plan: ResMut<UnitPlan>,
    mut ev_input: EventReader<InputEvent>,
    mut commands: Commands,
    arrow_atlas: Res<ArrowAtlas>,
    mut ev_plan_update: EventWriter<PlanUpdateEvent>,
    mut ev_confirm_move: EventWriter<ConfirmMoveEvent>,
) {
    'outer: for input_event in ev_input.read() {
        info!("Executing update_movement_plan");
        let (UnitId(unit_id), mut transform) = q_selected_unit.single_mut();

        let (dx, dy): (i32, i32) = match input_event {
            &InputEvent::Up => (0, 1),
            &InputEvent::Down => (0, -1),
            &InputEvent::Left => (-1, 0),
            &InputEvent::Right => (1, 0),
            &InputEvent::Select => {
                if let Some(last_step) = unit_plan.steps.last() {
                    if !scenario_state.is_tile_occupied(
                        *unit_id,
                        last_step.tile.x,
                        last_step.tile.y,
                    ) {
                        info!("Sending confirm move event!");
                        ev_confirm_move.send(ConfirmMoveEvent);
                    }
                }
                break 'outer;
            }
            _ => break, // Could add select here?
        };

        info!("dx, dy: {:?} {:?}", dx, dy);

        // TODO Split by event here?

        let tile = match check_valid(&transform, &scenario_state, *unit_id, (dx, dy)) {
            Some(tile) => tile,
            None => continue,
        };

        let plan_change: PlanChange = check_plan(&unit_plan, tile);

        match plan_change {
            PlanChange::Add(tile) => {
                add_tile(tile, &mut unit_plan, &arrow_atlas, &mut commands);
                update_transform(&mut transform, (dx, dy));
                ev_plan_update.send(PlanUpdateEvent);
            }
            PlanChange::Remove(index) => {
                remove_tile(&mut unit_plan, index, &mut commands);
                update_transform(&mut transform, (dx, dy));
                ev_plan_update.send(PlanUpdateEvent);
            }
            _ => {}
        }
        info!("Translation: {:?}", transform.translation);
    }
}

fn check_valid(
    unit_transform: &Transform,
    scenario_state: &ScenarioState,
    unit_id: u32,
    (dx, dy): (i32, i32),
) -> Option<Tile> {
    info!("checking valid");
    let unit_current_pos = Tile::from(*unit_transform);

    let valid_tiles = scenario_state.get_moveable_tiles(unit_id);
    let maybe_tile = valid_tiles
        .into_iter()
        .find(|tile| {
            tile.x as i32 == unit_current_pos.x as i32 + dx
                && tile.y as i32 == unit_current_pos.y as i32 + dy
        })
        .map(|EngineTile { x, y }| Tile { x, y });
    info!("validity checked");
    return maybe_tile;
}

fn check_plan(unit_plan: &ResMut<UnitPlan>, tile: Tile) -> PlanChange {
    info!("checking plan");
    // Don't allow overlapping vision.
    // if let Some(index) = unit_plan
    //     .steps
    //     .iter()
    //     .position(|move_step| move_step.tile == tile)
    // {
    // Allow overlapping vision
    if unit_plan.steps.len().checked_sub(2).map_or(false, |i| {
        info!("i: {:?}", i);
        unit_plan.steps[i].tile == tile
    }) {
        return PlanChange::Remove(unit_plan.steps.len() - 2);
    } else if (unit_plan.steps.len() as u32) <= unit_plan.range {
        return PlanChange::Add(tile);
    } else {
        return PlanChange::Invalid;
    }
}

fn add_tile(
    tile: Tile,
    unit_plan: &mut ResMut<UnitPlan>,
    arrow_atlas: &Res<ArrowAtlas>,
    commands: &mut Commands,
) {
    let entity = commands
        .spawn((
            MoveStepSprite,
            Transform::from_translation(Vec3::new(
                tile.x as f32 * TILE_SIZE,
                tile.y as f32 * TILE_SIZE,
                5.0,
            )),
            Sprite {
                image: arrow_atlas.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: arrow_atlas.layout.clone(),
                    index: 0,
                }),
                ..Default::default()
            },
            Visibility::Visible,
        ))
        .id();

    unit_plan.steps.push(MoveStep { tile, entity });
}

fn remove_tile(unit_plan: &mut ResMut<UnitPlan>, index: usize, commands: &mut Commands) {
    info!("removing tile");
    while unit_plan.steps.iter().len() > index + 1 {
        let popped_move = unit_plan.steps.pop().unwrap();
        commands.entity(popped_move.entity).despawn_recursive();
    }
}

fn update_transform(transform: &mut Transform, (dx, dy): (i32, i32)) {
    transform.translation.x += dx as f32 * TILE_SIZE;
    transform.translation.y += dy as f32 * TILE_SIZE;
    info!(
        "Transform x: {}, y: {}",
        transform.translation.x, transform.translation.y
    );
}

#[derive(Event)]
pub struct PlanUpdateEvent;

pub fn update_arrows(
    mut ev_plan_update: EventReader<PlanUpdateEvent>,
    unit_plan: Res<UnitPlan>,
    mut q_texture_atlas_sprite: Query<(&mut Visibility, &mut Sprite), With<MoveStepSprite>>,
) {
    for _ in ev_plan_update.read() {
        info!("Executing update_arrows");
        let len = unit_plan.steps.len();
        for i in 0..len {
            let MoveStep { tile, entity } = unit_plan.steps[i];

            let before_tile = if i == 0 {
                Option::None
            } else {
                unit_plan.steps.get(i - 1).map(|step| step.tile)
            };
            let after_tile = if i == len - 1 {
                Option::None
            } else {
                unit_plan.steps.get(i + 1).map(|step| step.tile)
            };

            let (mut visibility, mut sprite) = q_texture_atlas_sprite.get_mut(entity).unwrap();

            if let Some(atlas) = &mut sprite.texture_atlas {
                match get_index_from_tiles(before_tile, tile, after_tile) {
                    None => *visibility = Visibility::Hidden,
                    Some(atlas_index) => {
                        *visibility = Visibility::Visible;
                        atlas.index = atlas_index;
                    }
                }
            }
        }
    }
}

pub fn confirm_move(
    mut ev_input: EventReader<ConfirmMoveEvent>,
    mut ev_action: EventWriter<ActionEvent>,
    q_selected_unit: Query<Entity, (With<Selected>, With<UnitId>)>,
    unit_plan: Res<UnitPlan>,
) {
    for _ in ev_input.read() {
        info!("Executing confirm_move");

        let entity = q_selected_unit.single();
        info!("Sending ActionEvent!");
        ev_action.send(ActionEvent(Action::Move {
            entity,
            tiles: unit_plan.steps.iter().map(|step| step.tile).collect(),
        }));
    }
}

pub fn exit_movement_plan(mut unit_plan: ResMut<UnitPlan>, mut commands: Commands) {
    for step in unit_plan.steps.iter() {
        commands.entity(step.entity).despawn();
    }

    unit_plan.range = 0;
    unit_plan.steps = vec![];
}

pub fn get_index_from_tiles(
    before_tile: Option<Tile>,
    tile: Tile,
    after_tile: Option<Tile>,
) -> Option<usize> {
    let before_dir = before_tile.map_or(Dir::None, |before| get_direction(before, tile));
    let after_dir = after_tile.map_or(Dir::None, |after| get_direction(tile, after));
    return get_index_from_directions((before_dir, after_dir));
}

fn get_direction(tile_a: Tile, tile_b: Tile) -> Dir {
    if tile_b.x < tile_a.x {
        return Dir::Left;
    }
    if tile_b.x > tile_a.x {
        return Dir::Right;
    }
    if tile_b.y < tile_a.y {
        return Dir::Down;
    }
    if tile_b.y > tile_a.y {
        return Dir::Up;
    }
    panic!(
        "Tried to create arrow path between tiles which were not next to each other: {:?} and {:?}",
        tile_a, tile_b
    );
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn get_index_from_directions((from, to): (Dir, Dir)) -> Option<usize> {
    let index = match (from, to) {
        (Dir::Down, Dir::Down) => 11,
        (Dir::Down, Dir::Right) => 15,
        (Dir::Down, Dir::Left) => 14,
        (Dir::Down, Dir::None) => 17,
        // -
        (Dir::Up, Dir::Right) => 7,
        (Dir::Up, Dir::Up) => 12,
        (Dir::Up, Dir::None) => 6,
        (Dir::Up, Dir::Left) => 10,
        // -
        (Dir::Left, Dir::Up) => 13,
        (Dir::Left, Dir::Down) => 9,
        (Dir::Left, Dir::Left) => 22,
        (Dir::Left, Dir::None) => 21,
        // -
        (Dir::Right, Dir::Right) => 1,
        (Dir::Right, Dir::None) => 2,
        (Dir::Right, Dir::Down) => 8,
        (Dir::Right, Dir::Up) => 16,
        // -
        (Dir::None, Dir::Right) => 0,
        (Dir::None, Dir::Down) => 5,
        (Dir::None, Dir::Up) => 18,
        (Dir::None, Dir::Left) => 23,
        // -
        (Dir::Right, Dir::Left)
        | (Dir::Left, Dir::Right)
        | (Dir::Up, Dir::Down)
        | (Dir::Down, Dir::Up)
        | (Dir::None, Dir::None) => {
            return Option::None;
        }
    };
    return Option::Some(index);
}
