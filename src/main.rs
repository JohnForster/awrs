use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

mod awrs;

use awrs::game::AWRSPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin, AWRSPlugin))
        .add_systems(Startup, start_background_audio)
        .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audio/terran_1.mp3")).looped();
}
