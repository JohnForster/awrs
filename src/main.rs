use bevy::{
    prelude::*,
    window::{WindowPlugin, WindowResolution},
};

mod awrs;

use awrs::AWRSPlugin;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                            .with_scale_factor_override(2.0),
                        ..default()
                    }),
                    ..default()
                }),
            AWRSPlugin,
        ))
        .run();
}
