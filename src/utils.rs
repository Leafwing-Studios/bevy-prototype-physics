use bevy::prelude::*;

use crate::kinematics::KinematicsBundle;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(UiCameraBundle::default());
}

pub fn spawn_test_object(mut commands: Commands) {
    commands
        .spawn(SpriteBundle::default())
        .with_bundle(KinematicsBundle::default());
}
