use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

use super::game::*;
use super::sprite_loading::*;
use super::unit_loading::*;

pub struct LoadAssets;

pub struct AssetsLoading(pub Vec<HandleUntyped>);

impl Plugin for LoadAssets {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AssetsLoading(vec![]))
            .add_plugin(RonAssetPlugin::<UnitType>::new(&["ron"]))
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(load_terrain_sprites.system())
                    .with_system(load_unit_sprites.system())
                    .with_system(load_ui_sprites.system())
                    .with_system(load_units.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_assets_ready.system()),
            );
    }
}

pub fn check_assets_ready(
    loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<AppState>>,
    mut commands: Commands,
) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            // all assets are now ready
            commands.remove_resource::<AssetsLoading>();
            game_state.set(AppState::InGame(GameState::SetUp)).unwrap();
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
