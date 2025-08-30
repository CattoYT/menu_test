use bevy::prelude::*;

use crate::GameState;

pub struct SettingsMenu;

//current goal is to make it really ez for new settings so basically just a settings library fuck my life

// settings list rn:
// - vsync
// - fullscreen and borderless fullscreen (windowed will come later but id have to do some screen size calcs for the actual game ffs why dont i do what osu does)
// - offset (painpeko)
// - screen
// - keybinds (default to asd | jkl for 2key and a | s | k | l for 4key)

impl Plugin for SettingsMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), load_settings_menu);

    }
}
 
fn load_settings_menu() {

}