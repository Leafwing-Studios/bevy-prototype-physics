use bevy::prelude::*;
use bevy_prototype_physics::kinematics::*;

fn main() {
    let velocity = Velocity {
        val: Vec3::new(0.0, -50.0, 0.0),
    };
    let acceleration = Acceleration {
        val: Vec3::new(20.0, 0.0, 0.0),
    };

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(KinematicsPlugin)
        .insert_resource(velocity)
        .insert_resource(acceleration)
        .add_startup_system(setup_camera.system())
        .add_startup_system(spawn_test_object.system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_test_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    velocity: Res<Velocity>,
    acceleration: Res<Acceleration>,
) {
    let texture_handle = asset_server.load("bevy.png");

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .insert_bundle((Velocity::from(*velocity), Acceleration::from(*acceleration)));
}
