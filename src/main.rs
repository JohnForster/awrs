#![feature(mixed_integer_ops)]

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

mod awrs;

use awrs::game::AWRSPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(AWRSPlugin)
        .add_startup_system(start_background_audio)
        .add_startup_system(enable_hot_reload)
        .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("audio/terran_1.mp3"));
}

fn enable_hot_reload(asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}
