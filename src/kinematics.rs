use bevy::{core::FixedTimestep, prelude::*};

const PHYSICS_TICK: f32 = 1.0/120.0;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct Physics;

pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::new()
            .label(Physics)
            .with_run_criteria(FixedTimestep::step(PHYSICS_TICK.into()))
            .with_system(kinematics.system())
        );
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
) {
    for (acceleration, mut velocity, mut transform) in query.iter_mut() {
        velocity.val += acceleration.val * PHYSICS_TICK;
        transform.translation += velocity.val * PHYSICS_TICK;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_world(velocity: Velocity, acceleration: Acceleration) -> (World, SystemStage, Entity) {
        let mut world = World::default();

        let update_stage = SystemStage::parallel();

        let entity_id = world
            .spawn()
            .insert_bundle(SpriteBundle::default())
            .insert_bundle((Velocity::from(velocity), Acceleration::from(acceleration)))
            .id();

        (world, update_stage, entity_id)
    }

    #[test]
    fn linear_motion_test() {
        let (mut world, mut update_stage, entity_id) = setup_world(
            Velocity {
                val: Vec3::new(10.0, 0.0, 0.0),
            },
            Acceleration::default(),
        );

        update_stage.add_system(kinematics.system());
        update_stage.run(&mut world);

        assert_eq!(
            world.get::<Velocity>(entity_id).unwrap().val.x, 
            10.0
        );
        assert_eq!(
            world.get::<Transform>(entity_id).unwrap().translation.x, 
            10.0 * PHYSICS_TICK
        );
    }

    #[test]
    fn linear_acceleration_test() {
        let (mut world, mut update_stage, entity_id) = setup_world(
            Velocity::default(),
            Acceleration { val: Vec3::new(0.0, 7.0, 0.0) },
        );

        update_stage.add_system(kinematics.system());
        update_stage.run(&mut world);

        assert_eq!(
            world.get::<Velocity>(entity_id).unwrap().val.y, 
            7.0 * PHYSICS_TICK
        );
        assert_eq!(
            world.get::<Transform>(entity_id).unwrap().translation.y, 
            7.0 * PHYSICS_TICK * PHYSICS_TICK
        );
    }
}
