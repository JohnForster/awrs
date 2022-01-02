#![feature(mixed_integer_ops)]

use bevy::prelude::*;

mod awrs;

use awrs::game::AWRSPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AWRSPlugin)
        .run();
}
