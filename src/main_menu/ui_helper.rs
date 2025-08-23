use bevy::prelude::*;

use crate::main_menu::ButtonAction;

const BUTTON_ROUNDING: f32 = 18.;

const BASE_BUTTON_COLOUR: Color = Color::srgb_u8(200, 138, 246);

pub fn spawn_main_menu(asset_server: &AssetServer) -> impl Bundle {
    (
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
        BackgroundColor(Color::srgb_u8(26, 22, 30)),
        children![
            ( // title 
                Text::new("Rhythm_game:tm:"),
                TextFont {
                    font: asset_server.load("./fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            ( // start button
                Node {
                    top: Val::Percent(40.), //only jesus knows where these values came from (my ass)
                    left: Val::Percent(-39.),
                    width: Val::Percent(10.),
                    height: Val::Percent(10.),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::new(
                    // top left
                    Val::Px(BUTTON_ROUNDING),
                    // top right
                    Val::Px(BUTTON_ROUNDING),
                    // bottom right
                    Val::Px(BUTTON_ROUNDING),
                    // bottom left
                    Val::Px(BUTTON_ROUNDING),
                ),
                Button,
                BackgroundColor(BASE_BUTTON_COLOUR),
                ButtonAction::Start,
                children![(
                    Text::new("Start"),
                    TextFont {
                        font: asset_server.load("./fonts/FiraSans-Bold.ttf"),
                        ..default()
                    }
                )]
            ),
            ( //settings
                Node {
                    top: Val::Percent(40.), //only jesus knows where these values came from (my ass)
                    left: Val::Percent(-39.),
                    width: Val::Percent(10.),
                    height: Val::Percent(10.),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::new(
                    // top left
                    Val::Px(BUTTON_ROUNDING),
                    // top right
                    Val::Px(BUTTON_ROUNDING),
                    // bottom right
                    Val::Px(BUTTON_ROUNDING),
                    // bottom left
                    Val::Px(BUTTON_ROUNDING),
                ),
                Button,
                ButtonAction::Settings,
                BackgroundColor(BASE_BUTTON_COLOUR),
                children![(
                    Text::new("Settings"),
                    TextFont {
                        font: asset_server.load("./fonts/FiraSans-Bold.ttf"),
                        ..default()
                    }
                )]
            ),
            (// quit button
                Node {
                    top: Val::Percent(40.), //only jesus knows where these values came from (my ass)
                    left: Val::Percent(-39.),
                    width: Val::Percent(10.),
                    height: Val::Percent(10.),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::new(
                    // top left
                    Val::Px(BUTTON_ROUNDING),
                    // top right
                    Val::Px(BUTTON_ROUNDING),
                    // bottom right
                    Val::Px(BUTTON_ROUNDING),
                    // bottom left
                    Val::Px(BUTTON_ROUNDING),
                ),
                Button,
                ButtonAction::Quit,
                BackgroundColor(BASE_BUTTON_COLOUR),
                children![(
                    Text::new("Quit"),
                    TextFont {
                        font: asset_server.load("./fonts/FiraSans-Bold.ttf"),
                        ..default()
                    }
                )]
            ),
        ],
    )
}
