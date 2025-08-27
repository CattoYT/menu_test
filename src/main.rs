use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod game;
mod main_menu;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Settings,
    MainMenu,
    InGame,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(main_menu::MainMenu)
        .add_plugins(game::Game)
        .insert_state(GameState::MainMenu)
        .run()
}
