use crate::{settings::Settings, GameState};

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub(in crate::ui) enum ButtonAction {
    Start,
    OpenSettings,
    ModifySetting, // TODO: MAKE THIS WORK
    OpenMainMenu,
    Quit,
}

// on press stuff
pub(in crate::ui) fn button_fucker(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in query {

        match *interaction {
            Interaction::Pressed => {
                println!("a");
                button_manager(&mut next_state, 
                    action);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}


// button thing does what the button wants to button
pub(in crate::ui) fn button_manager(next_state: &mut ResMut<NextState<GameState>>, button_type: &ButtonAction) {
    match *button_type {
        ButtonAction::Start => {
            next_state.set(GameState::InGame);
        }
        ButtonAction::OpenSettings => next_state.set(GameState::Settings),
        ButtonAction::ModifySetting => todo!(),
        ButtonAction::OpenMainMenu => next_state.set(GameState::MainMenu),
        ButtonAction::Quit => std::process::exit(0),
    }
    info!("Migrated state to {:?}", *button_type);
}

