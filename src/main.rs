use bevy::{math::vec3, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(move_camera)
        .insert_resource(MoveTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<SpawnBundle>("Spawn")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("world.ldtk"),
        ..Default::default()
    });
}

#[derive(Resource)]
struct MoveTimer(Timer);

fn move_camera(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut spawn: Query<&GridCoords, With<SpawnPosition>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let spawn_coords: Vec<&GridCoords> = spawn.iter().collect();
    let mut rng = thread_rng();

    let value = SliceRandom::choose((&spawn_coords) as &[&GridCoords], &mut rng);

    if let Some(spawn) = value {
        println!("{:?}", spawn);
        for mut transform in camera.iter_mut() {
            transform.translation.x = spawn.x as f32 * 32.0;
            transform.translation.y = spawn.y as f32 * 32.0;
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct SpawnBundle {
    #[grid_coords]
    grid_coords: GridCoords,

    spawn: SpawnPosition,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Component)]
pub struct SpawnPosition;
