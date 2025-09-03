use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
};
use rand::Rng;

use crate::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameplaySet;

pub struct Game;
impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), init_game_scene);

        app.add_systems(
            FixedUpdate,
            (
                spawn_note.run_if(on_timer(Duration::from_secs(1))),
                (
                    clear_offscreen_note, 
                    move_note_down
                )
                    //universal things

            ).in_set(GameplaySet)
            .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component, Debug, Clone)]
struct Ball {
    lane: i8,
    timestamp: f32,
}

impl Default for Ball {
    fn default() -> Ball {
        Ball { lane: 0, timestamp: 0.0 }
    }
}

fn init_game_scene(mut commands: Commands) {
    commands.spawn(Camera3d::default());
    commands.spawn(DirectionalLight::default());
}

fn spawn_note(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let ball_mesh = mesh_assets.add(Sphere::new(2.));
    let ball_material = material_assets.add(StandardMaterial {
        base_color: Color::srgb(30., 60., 99.),
        ..Default::default()
    });
    let temp_lane = rand::rng().random_range(0..=4);
    commands.spawn((
        Ball { lane: temp_lane, ..default() },
        Transform::from_translation(Vec3 {
            x: 30.,
            y: -1. + temp_lane as f32 * 6.,
            z: -50.,
        }),
        Mesh3d(ball_mesh.clone()),
        MeshMaterial3d(ball_material.clone()),
    ));
    println!("Spawned ball in lane: {}", temp_lane);
}

fn move_note_down(query: Query<&mut Transform, With<Ball>>) {
    for mut ball in query {
        ball.translation.x -= 0.3;
    }
}

fn clear_offscreen_note(
    mut commands: Commands,
    query: Query<(Entity, &ViewVisibility), With<Ball>>,
) {
    for (note, visible) in query {
        if !**visible {
            commands.entity(note).despawn();
            info!("Note despawned");
        }
    }
}
