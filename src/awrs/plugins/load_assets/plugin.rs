use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

use crate::awrs::resources::state::{AppState, GameState};

use super::sprite_loading::*;
use super::unit_loading::*;

pub struct LoadAssetsPlugin;

pub struct AssetsLoading(pub Vec<HandleUntyped>);

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoading(vec![]))
            .add_plugin(RonAssetPlugin::<UnitStats>::new(&["unit.ron"]))
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(load_images)
                    .with_system(create_terrain_sprites) // Move to setup
                    .with_system(create_idle_sprites) // Move to setup
                    .with_system(create_movement_arrow_sprites) // Move to setup
                    .with_system(create_ui_sprites) // Move to setup
                    .with_system(load_units),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_assets_ready),
            );
    }
}

pub fn check_assets_ready(
    loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<State<AppState>>,
    mut st_game: ResMut<State<GameState>>,
    mut commands: Commands,
) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            // all assets are now ready
            commands.remove_resource::<AssetsLoading>();
            app_state.set(AppState::InGame).unwrap();
            st_game.set(GameState::SetUp).unwrap();
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
