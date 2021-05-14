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

    fn setup_world(velocity: Velocity, acceleration: Acceleration) -> (World, SystemStage, Entity) {
        let mut world = World::default();
        world.insert_resource(Time::default());
        
        let update_stage = SystemStage::parallel();

        let entity_id = world.spawn()
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
            Acceleration { val: Vec3::ZERO },
        );

        update_stage.add_system(kinematics.system());
        update_stage.run(&mut world);

        // TODO: write a more precise test
        assert!(world.get::<Velocity>(entity_id).unwrap().val.x < 0.0);
        
    }
}
