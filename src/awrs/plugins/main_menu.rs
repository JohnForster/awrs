use advance_craft_server::{ClientToServer, ServerToClient};
use bevy::prelude::*;

use crate::awrs::{
    register_inputs::InputEvent,
    resources::{
        client::{ReceiveWebsocketMessageEvent, SendWebsocketMessageEvent},
        scenario::ScenarioState,
        start_game::StartGameEvent,
        state::AppState,
    },
};

pub struct MainMenuPlugin;

#[derive(Resource)]
struct Selected(u32);

#[derive(Event)]
struct SelectOptionEvent(u32);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Selected(0))
            .add_event::<SelectOptionEvent>()
            .add_event::<StartGameEvent>()
            .add_systems(
                OnEnter(AppState::MainMenu),
                (setup_main_menu, connect_to_server),
            )
            .add_systems(Update, (handle_navigation, change_menu, load_game))
            .add_systems(OnExit(AppState::MainMenu), teardown_main_menu);
    }
}

use crate::awrs::resources::client::WebSocketConnectionEvents;

pub fn connect_to_server(mut ev_ws_connect: EventWriter<WebSocketConnectionEvents>) {
    ev_ws_connect.send(WebSocketConnectionEvents::SetupConnection {});
}

#[derive(Component)]
struct MenuOption(u32);

#[derive(Component)]
struct MenuContainer;

#[derive(Component)]
struct MenuTitle;

#[derive(Component)]
struct MenuCamera;

fn setup_main_menu(mut commands: Commands) {
    // Root node
    commands.spawn((
        Camera2d::default(),
        OrthographicProjection::default_2d(),
        MenuCamera,
    ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            MenuContainer,
        ))
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            parent.spawn((
                Text::new("Advance Craft II"),
                TextFont {
                    font_size: 36.0,
                    // font: _asset_server.load("fonts/aw2-gba.otf"),
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                MenuTitle,
            ));

            create_option(parent, "New Game", 0);
            create_option(parent, "Join Game", 1);
            create_option(parent, "Play Solo", 2);
        });
}

fn create_option(parent: &mut ChildBuilder<'_>, text: &str, index: u32) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: 24.0,
            // font: asset_server.load("fonts/aw2-gba.otf"),
            ..Default::default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        MenuOption(index),
    ));
}

fn change_menu(
    mut ev_select_option: EventReader<SelectOptionEvent>,
    mut ev_websocket: EventWriter<SendWebsocketMessageEvent>,
) {
    for event in ev_select_option.read() {
        match event.0 {
            0 => {
                ev_websocket.send(SendWebsocketMessageEvent::from(
                    ClientToServer::CreateGame {},
                ));
                // Replace menu with "loading" indicator.
                info!("New Game");
            }
            1 => {
                info!("Join Game");
            }
            2 => {
                info!("Play Solo");
            }
            _ => {}
        }
    }
}

fn load_game(
    mut ev_ws_message: EventReader<ReceiveWebsocketMessageEvent>,
    mut ev_start_game: EventWriter<StartGameEvent>,
) {
    for event in ev_ws_message.read() {
        let message = event.try_into_data::<ServerToClient>().unwrap();
        match message {
            ServerToClient::CreateGameResult {
                game_id,
                scenario_state,
            } => {
                ev_start_game.send(StartGameEvent {
                    game_id: Some(game_id.to_string()),
                    scenario_state: ScenarioState(scenario_state),
                    online: true,
                });
            }
            _ => {}
        }
    }
}

fn handle_navigation(
    mut q_menu_options: Query<(&mut TextColor, &MenuOption)>,
    mut selected: ResMut<Selected>,
    mut ev_input: EventReader<InputEvent>,
    mut ev_select_option: EventWriter<SelectOptionEvent>,
) {
    for input in ev_input.read() {
        match input {
            InputEvent::Down => {
                selected.0 = (selected.0 + 1) % 3;
            }
            InputEvent::Up => {
                selected.0 = (selected.0 + 2) % 3;
            }
            InputEvent::Select => {
                ev_select_option.send(SelectOptionEvent(selected.0));
            }
            _ => {}
        }
    }

    for (mut text_color, option) in q_menu_options.iter_mut() {
        if option.0 == selected.0 {
            text_color.0 = Color::srgb(0.9, 0.9, 0.9);
        } else {
            text_color.0 = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}

fn teardown_main_menu(
    q_menu_container: Query<Entity, With<MenuContainer>>,
    q_menu_camera: Query<Entity, With<MenuCamera>>,
    mut commands: Commands,
) {
    for entity in q_menu_container.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in q_menu_camera.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
