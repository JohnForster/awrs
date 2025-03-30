use bevy::{
    prelude::*,
    window::{WindowPlugin, WindowResolution},
};

mod awrs;

use awrs::{AWRSPlugin, AppConfig};

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ApplicationArgs {
    #[arg(long, short = 'x', default_value = "1280")]
    pub window_width: f32,
    #[arg(long, short = 'y', default_value = "720")]
    pub window_height: f32,
    #[arg(long, short = 's', default_value = "127.0.0.1:8080/")]
    pub server_address: String,

    #[arg(long, short = 'g', default_value = None)]
    pub game_id: Option<String>,
}

fn main() {
    let args = ApplicationArgs::parse();
    dbg!(&args.game_id);

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
        .insert_resource(AppConfig {
            window_width: args.window_width,
            window_height: args.window_height,
            server_address: args.server_address,
            game_id: args.game_id.and_then(|id| id.parse().ok()),
        })
        .run();
}
