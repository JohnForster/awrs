use bevy::prelude::*;

mod awrs;

use awrs::game::AWRSPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AWRSPlugin,
        ))
        .run();
}
