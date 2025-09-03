use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod game;
mod settings;
mod ui;
// mod main_menu;
// mod settings;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Settings,
    MainMenu,
    InGame,
}

fn main() -> AppExit {
    let mut config = match settings::get_config_from_file() {
        Ok(config) => config,
        Err(e) => panic!("Config failed to load! Reason: {}", e),
    };

    App::new()
        .add_plugins(
            //indented like this for toggling later cuz settings arent done yet and ui is difficult
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: config.screen_type.into(), //i could *probably* use a ref here but idk how cuz im bad lmao
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ui::main_menu::MainMenu)
        .add_plugins(ui::settings_menu::SettingsMenu)
        .add_plugins(game::Game)
        
        .insert_state(GameState::MainMenu)

        .insert_resource(config)
        .insert_resource(ClearColor(Color::srgb_u8(26, 22, 30)))
        .run()
}
