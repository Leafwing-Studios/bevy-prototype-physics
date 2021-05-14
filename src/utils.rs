use bevy::prelude::*;

use crate::kinematics::{Acceleration, Velocity};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn spawn_test_object(mut commands: Commands, velocity: Res<Velocity>, acceleration: Res<Acceleration>) {
    commands
        .spawn_bundle(SpriteBundle::default())
        .insert_bundle((Velocity::from(*velocity), Acceleration::from(*acceleration)));
}
