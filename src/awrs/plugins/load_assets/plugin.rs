use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::awrs::resources::state::{AppState, GameState};

use super::sprite_loading::*;
use super::unit_loading::*;

pub struct LoadAssetsPlugin;

#[derive(bevy::prelude::Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct LoadingSet;

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoading(vec![]))
            .add_plugins(RonAssetPlugin::<UnitStats>::new(&["unit.ron"]))
            .configure_sets(Update, LoadingSet.run_if(in_state(AppState::Loading)))
            .add_systems(
                OnEnter(AppState::Loading),
                (
                    load_images,
                    create_terrain_sprites,        // Move to setup
                    create_idle_sprites,           // Move to setup
                    create_movement_arrow_sprites, // Move to setup
                    create_ui_sprites,
                    create_creep_sprites, // Move to setup
                    load_units,
                ),
            )
            .add_systems(Update, check_assets_ready.in_set(LoadingSet));
    }
}

pub fn check_assets_ready(
    loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let mut loading_states = loading
        .0
        .iter()
        .map(|h| asset_server.get_load_state(h.id()));

    let all_complete = loading_states.all(|opt| opt.is_some());

    if all_complete {
        // all loading is complete (it is possible loading failed)
        commands.remove_resource::<AssetsLoading>();
        next_app_state.set(AppState::InGame);
        next_game_state.set(GameState::SetUp);
    }
}
