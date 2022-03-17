use bevy::prelude::*;

use super::arrows::get_index_from_tiles;

use crate::awrs::{
    engine::{ScenarioState, Tile as EngineTile},
    register_inputs::InputEvent,
    resources::{
        action_event::{Action, ActionEvent, ActionResultEvent},
        atlases::ArrowAtlas,
        cursor::{ChangeCursorEvent, Cursor, CursorStyle},
        state::GameState,
        tile::{Tile, TILE_SIZE},
        unit::{Selected, UnitId},
    },
};

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

    let (UnitId(unit_id), transform) = q_selected_unit.single_mut();

    let range = scenario_state.get_movement_range(&unit_id);
    unit_plan.range = range;
    unit_plan.steps = vec![];

    if let Some(tile) = check_valid(&transform, &scenario_state, *unit_id, (0, 0)) {
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
    'outer: for input_event in ev_input.iter() {
        info!("Executing update_movement_plan");
        let (UnitId(unit_id), mut transform) = q_selected_unit.single_mut();

        let (dx, dy): (i32, i32) = match input_event {
            &InputEvent::Up => (0, 1),
            &InputEvent::Down => (0, -1),
            &InputEvent::Left => (-1, 0),
            &InputEvent::Right => (1, 0),
            &InputEvent::Select => {
                info!("Sending confirm move event!");
                ev_confirm_move.send(ConfirmMoveEvent);
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
    transform: &Transform,
    scenario_state: &ScenarioState,
    unit_id: u32,
    (dx, dy): (i32, i32),
) -> Option<Tile> {
    info!("checking valid");
    let unit_current_pos = Tile::from(*transform);

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
    info!("adding tile");
    let sprite = TextureAtlasSprite::new(0);

    let entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: arrow_atlas.atlas_handle.clone(),
            sprite,
            visibility: Visibility { is_visible: false },
            transform: Transform::from_translation(Vec3::new(
                tile.x as f32 * TILE_SIZE,
                tile.y as f32 * TILE_SIZE,
                5.0,
            )),
            ..Default::default()
        })
        .insert(MoveStepSprite)
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

pub struct PlanUpdateEvent;

pub fn update_arrows(
    mut ev_plan_update: EventReader<PlanUpdateEvent>,
    unit_plan: Res<UnitPlan>,
    mut q_texture_atlas_sprite: Query<
        (&mut TextureAtlasSprite, &mut Visibility),
        With<MoveStepSprite>,
    >,
) {
    for _ in ev_plan_update.iter() {
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

            let (mut move_step_sprite, mut visibility) =
                q_texture_atlas_sprite.get_mut(entity).unwrap();

            match get_index_from_tiles(before_tile, tile, after_tile) {
                None => {
                    visibility.is_visible = false;
                }
                Some(sprite_index) => {
                    visibility.is_visible = true;
                    move_step_sprite.index = sprite_index;
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
    for _ in ev_input.iter() {
        info!("Executing confirm_move");

        let entity = q_selected_unit.single();
        info!("Sending ActionEvent!");
        ev_action.send(ActionEvent(Action::Move {
            entity,
            tiles: unit_plan.steps.iter().map(|step| step.tile).collect(),
        }))
    }
}

pub fn move_result(
    mut ev_move_result: EventReader<ActionResultEvent>,
    mut st_game: ResMut<State<GameState>>,
    mut q: QuerySet<(
        QueryState<&mut Transform, With<Selected>>,
        QueryState<&mut Transform, With<Cursor>>,
    )>,
) {
    for action_result in ev_move_result.iter() {
        info!("Executing move_result");
        if let ActionResultEvent::MoveResult(tiles) = action_result {
            if let Some(location) = tiles.last() {
                info!("Moving unit...");

                let mut unit_query = q.q0();
                let mut unit_transform = unit_query.single_mut();
                unit_transform.translation.x = location.x as f32 * TILE_SIZE;
                unit_transform.translation.y = location.y as f32 * TILE_SIZE;

                let mut cursor_query = q.q1();
                let mut cursor_transform = cursor_query.single_mut();
                cursor_transform.translation.x = location.x as f32 * TILE_SIZE;
                cursor_transform.translation.y = location.y as f32 * TILE_SIZE;
            } else {
            }

            st_game
                .set(GameState::Browsing)
                .expect("Problem changing state");
        }
    }
}

pub fn exit_movement_plan(mut unit_plan: ResMut<UnitPlan>, mut commands: Commands) {
    for step in unit_plan.steps.iter() {
        commands.entity(step.entity).despawn();
    }

    unit_plan.range = 0;
    unit_plan.steps = vec![];
}
