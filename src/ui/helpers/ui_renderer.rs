use bevy::prelude::*;

use crate::ui::{BASE_BUTTON_COLOUR, BUTTON_ROUNDING, helpers::buttons::ButtonAction};

pub struct UIPosition {
    pub top: f32, //only jesus knows where these values came from (my ass)
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

pub fn draw_simple_rounded_button(
    asset_server: &AssetServer,
    position: UIPosition,
    text: String,
    action: ButtonAction,
) -> impl Bundle {
    (
        Node {
            top: Val::Percent(position.top), //only jesus knows where these values came from (my ass)
            left: Val::Percent(position.left),
            width: Val::Percent(position.width),
            height: Val::Percent(position.height),
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
        action,
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load("./fonts/FiraSans-Bold.ttf"),
                ..default()
            }
        )],
    )
}
