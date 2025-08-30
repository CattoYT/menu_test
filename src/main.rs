use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod game;
mod main_menu;
mod settings;


#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Settings,
    MainMenu,
    InGame,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(
        //indented like this for toggling later cuz settings arent done yet and ui is difficult
        DefaultPlugins

            .set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),..default()
        })
    
    )
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(main_menu::MainMenu)
        .add_plugins(settings::SettingsMenu)
        .add_plugins(game::Game)

        .insert_state(GameState::MainMenu)

        .insert_resource(ClearColor(Color::srgb_u8(26, 22, 30)))

        .run()
}
