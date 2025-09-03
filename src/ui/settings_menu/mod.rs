use bevy::{prelude::*, window::PresentMode};

use crate::{
    GameState,
    ui::helpers::buttons::ButtonAction,
    ui::helpers::ui_renderer::{UIPosition, draw_simple_rounded_button},
};

pub struct SettingsMenu;

struct SettingsMenuComponent;

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
        app.add_systems(OnExit(GameState::Settings), despawn_settings_menu);
    }
}

fn load_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3d::default()));
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, // stack children vertically

            // // horizontal alignment
            justify_self: JustifySelf::Start,
            // // vertical alignment for future ref
            align_items: AlignItems::Center,

            ..Default::default()
        },
        children![
            (
                // title
                Text::new("Settings"),
                TextFont {
                    font: asset_server.load("./fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            draw_simple_rounded_button(
                &asset_server,
                UIPosition {
                    top: 40.,
                    left: -39.,
                    width: 10.,
                    height: 5.,
                },
                "Vsync".to_string(),
                ButtonAction::ModifySetting
            ),
            draw_simple_rounded_button(
                &asset_server,
                UIPosition {
                    top: 60.,
                    left: -39.,
                    width: 10.,
                    height: 5.,
                },
                "Back".to_string(),
                ButtonAction::OpenMainMenu, //buttons dont button
            ),
        ],
    ));
}

fn despawn_settings_menu() {}
