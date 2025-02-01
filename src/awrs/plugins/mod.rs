mod in_game;
pub use in_game::InGamePlugin;

mod load_assets;
pub use load_assets::LoadAssetsPlugin;

mod client;
pub use client::WebsocketClientPlugin;

mod main_menu;
pub use main_menu::MainMenuPlugin;
