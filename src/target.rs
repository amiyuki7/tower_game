use crate::*;
use bevy::math::Vec3Swizzles;

#[derive(Component, Reflect, Default)]
pub struct Target {
    pub speed: f32,
    pub path_index: usize,
}

#[derive(Resource)]
pub struct TargetPath {
    pub waypoints: Vec<Vec2>,
}

#[derive(Component, Reflect)]
pub struct Health {
    pub value: i32,
}

// Can have attached data
pub struct TargetDeathEvent;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_event::<TargetDeathEvent>()
            .insert_resource(TargetPath {
                waypoints: vec![
                    Vec2::new(6.0, 2.0),
                    Vec2::new(6.0, 6.0),
                    Vec2::new(9.0, 9.0),
                ],
            })
            .add_systems((move_targets, target_death).in_set(OnUpdate(GameState::GamePlay)));
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut death_event_writer: EventWriter<TargetDeathEvent>,
) {
    for (entity, health) in &targets {
        if health.value <= 0 {
            death_event_writer.send(TargetDeathEvent);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_targets(
    mut targets: Query<(&mut Target, &mut Transform)>,
    path: Res<TargetPath>,
    time: Res<Time>,
) {
    // for (target, mut transform) in &mut targets {
    //     transform.translation.x += target.speed * time.delta_seconds();
    // }

    for (mut target, mut transform) in &mut targets {
        let delta = target.speed * time.delta_seconds();
        let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

        if delta_target.length() > delta {
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();

            let y = transform.translation.y;
            transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y);
        } else {
            target.path_index += 1;
        }
    }
}
