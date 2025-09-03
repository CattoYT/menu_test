use std::error::Error;

use bevy::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Resource, Debug)]
pub struct Settings {
    pub(crate) screen_type: DisplayMode,
    song_offset: f32,
    vsync: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum DisplayMode {
    Windowed,
    BorderlessFullscreen,
}

impl From<DisplayMode> for bevy::window::WindowMode {
    fn from(d: DisplayMode) -> Self {
        match d {
            DisplayMode::Windowed => bevy::window::WindowMode::Windowed,
            DisplayMode::BorderlessFullscreen => {
                bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
            }
        }
    }
}

fn save_config() {}

pub(crate) fn get_config_from_file() -> Result<Settings, Box<dyn Error>> {
    let config = match std::fs::read("config.toml") {
        Ok(config) => config,

        Err(e) => match std::fs::exists("config.toml") {
            Ok(idk) => {
                if idk {
                    return Err(Box::new(e));
                } else {
                    let default_config = Settings {
                        screen_type: DisplayMode::BorderlessFullscreen,
                        song_offset: 0.0,
                        vsync: true,
                    };
                    let toml_string = toml::to_string(&default_config)?;
                    std::fs::write("config.toml", toml_string)?;
                    return Ok(default_config);
                }
            }

            Err(_) => {
                panic!("The fuck happened???")
            }
        },
    };
    let settings: Settings = toml::from_slice(&config)?;
    Ok(settings)
}
