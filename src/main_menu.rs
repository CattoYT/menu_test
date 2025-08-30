use bevy::prelude::*;

use crate::GameState;

mod ui_helper;

#[derive(Component, Debug, Clone)]
enum ButtonAction {
    Start,
    Settings,
    Quit,
}

#[derive(Component)]
struct MainMenuComponent;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), init_main_menu);
        app.add_systems(
            Update,
            (button_fucker).run_if(in_state(GameState::MainMenu)),
        );
        app.add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

fn init_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    println!("Main menu stated");
    commands.spawn((Camera3d::default(), MainMenuComponent));
    commands.spawn((ui_helper::spawn_main_menu(&assets), MainMenuComponent));
}


// on press stuff
fn button_fucker(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in query {

        match *interaction {
            Interaction::Pressed => {
                
                button_manager(&mut next_state, action);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}


// button thing does what the button wants to button
fn button_manager(next_state: &mut ResMut<NextState<GameState>>, button_type: &ButtonAction) {
    match *button_type {
        ButtonAction::Start => {
            next_state.set(GameState::InGame);
        }
        ButtonAction::Settings => next_state.set(GameState::Settings),
        ButtonAction::Quit => std::process::exit(0),
    }
    info!("Migrated state");
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MainMenuComponent>>) {
    for menu_component in query {
        commands.entity(menu_component).despawn();
    }
}
