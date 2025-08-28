use std::time::Duration;

use bevy::{
    ecs::{query, system::command},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameplaySet;

pub struct Game;
impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), init_game_scene);
        app.add_systems(
            Update,
            (
                test,
                clear_offscreen_note,
                spawn_note.run_if(on_timer(Duration::from_secs(1))),
            )
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
    commands.spawn((
        Ball { lane: 0 },
        Transform::from_translation(Vec3 {
            x: -30.,
            y: 5.,
            z: -50.,
        }),
        Mesh3d(ball_mesh.clone()),
        MeshMaterial3d(ball_material.clone()),
    ));
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
