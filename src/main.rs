use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod main_menu;


type GameState = main_menu::GameState;

fn main() -> AppExit {
    App::new()
    
    .add_plugins(DefaultPlugins)
    .insert_state(GameState::MainMenu)
    .add_plugins(main_menu::MainMenu)
    .add_plugins(EguiPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .run()
}
