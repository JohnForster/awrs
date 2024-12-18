use bevy::{
    prelude::*,
    window::{WindowPlugin, WindowResolution},
};

mod awrs;

use awrs::game::AWRSPlugin;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 800.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
            AWRSPlugin,
        ))
        .run();
}
