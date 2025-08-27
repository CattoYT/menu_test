use bevy::{
    ecs::{query, system::command},
    prelude::*,
};

use crate::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameplaySet;

pub struct Game;
impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (test, clear_offscreen_note)
                .in_set(GameplaySet)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn test() {
    println!("Game loopin");
}

#[derive(Component, Debug, Clone)]
struct Ball {
    lane: i8,
}

fn spawn_note(mut commands: Commands) {
    commands.spawn((Ball { lane: 0 }));
}

fn clear_offscreen_note(
    mut commands: Commands,
    query: Query<(Entity, &ViewVisibility), With<Ball>>,
) {
    for (note, visible) in query {
        if !**visible {
            commands.entity(note).despawn();
        }
    }
}
