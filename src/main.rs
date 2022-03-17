#![feature(mixed_integer_ops)]

use bevy::prelude::*;

mod awrs;

use awrs::game::AWRSPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AWRSPlugin)
        .run();
}
