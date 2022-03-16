use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

use crate::awrs::resources::ron_data::UnitSpriteDataList;
use crate::awrs::resources::state::{AppState, GameState};

use super::sprite_loading::*;
use super::unit_loading::*;

pub struct LoadAssets;

pub struct AssetsLoading(pub Vec<HandleUntyped>);

impl Plugin for LoadAssets {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AssetsLoading(vec![]))
            .add_plugin(RonAssetPlugin::<UnitType>::new(&["unit.ron"]))
            .add_plugin(RonAssetPlugin::<UnitSpriteDataList>::new(&[
                "unitsprite.ron",
            ]))
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(load_images.system())
                    .with_system(load_spritesheet_data.system()) // Move to setup
                    .with_system(create_terrain_sprites.system()) // Move to setup
                    .with_system(create_unit_sprites.system()) // Move to setup
                    .with_system(create_movement_arrow_sprites.system()) // Move to setup
                    .with_system(create_ui_sprites.system()) // Move to setup
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
