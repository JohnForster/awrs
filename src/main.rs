#![feature(mixed_integer_ops)]

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

mod awrs;

use awrs::game::AWRSPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin, AWRSPlugin))
        .add_systems(Startup, (enable_hot_reload, start_background_audio))
        .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("audio/terran_1.mp3"));
}

fn enable_hot_reload(asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}
