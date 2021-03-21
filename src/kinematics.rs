use bevy::prelude::*;

pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(kinematics.system());
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Velocity {
    pub val: Vec3,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Acceleration {
    pub val: Vec3,
}

/// Add to entities that already have a Transform component
#[derive(Bundle, Clone, Default, Debug)]
pub struct KinematicsBundle {
    velocity: Velocity,
    acceleration: Acceleration,
}

pub fn kinematics(
    mut query: Query<(&Acceleration, &mut Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (acceleration, mut velocity, mut transform) in query.iter_mut() {
        velocity.val += acceleration.val * delta_time;
        transform.translation += velocity.val * delta_time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{setup_camera, spawn_test_object};
    use bevy::prelude::*;

    fn setup_app(velocity: Velocity, acceleration: Acceleration) -> AppBuilder {
        fn set_velocity(mut query: Query<&mut Velocity>, velocity: Res<Velocity>) {
            for v in query.iter_mut() {
                // v = velocity;
            }
        }

        fn set_acceleration(mut query: Query<&mut Acceleration>, acceleration: Res<Acceleration>) {
            for a in query.iter_mut() {
                // a = acceleration;
            }
        }

        *App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(KinematicsPlugin)
            .insert_resource(Velocity { val: velocity })
            .insert_resource(Acceleration { val: acceleration })
            .add_startup_system(spawn_test_object.system())
            .add_startup_system_to_stage(StartupStage::PostStartup, set_velocity.system())
            .add_startup_system_to_stage(StartupStage::PostStartup, set_acceleration.system())
            .add_system(setup_camera.system())
    }

    #[test]
    fn linear_motion_test() {
        setup_app(
            Velocity {
                val: Vec3::new(10.0, 0.0, 0.0),
            },
            Acceleration { val: Vec3::ZERO },
        )
        .run();
    }

    #[test]
    fn constant_acceleration_test() {
        setup_app().run();
    }
}
