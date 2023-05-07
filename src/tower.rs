use crate::*;
use bevy::utils::FloatOrd;

#[derive(Component, Reflect)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
    pub range: f32,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        // app.register_type::<Tower>().add_system(tower_shooting)
        // // .add_system(build_tower);
        app.register_type::<Tower>()
            // .register_inspectable::<TowerType>()
            .add_systems(
                (
                    tower_shooting,
                    tower_button_clicked,
                    create_ui_on_selection,
                    grey_tower_buttons.after(create_ui_on_selection),
                )
                    .in_set(OnUpdate(GameState::GamePlay)),
            );
    }
}

fn tower_shooting(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, tower_type, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());

        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .filter(|target_transform| {
                    Vec3::distance(target_transform.translation(), bullet_spawn) < tower.range
                })
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                let (model, bullet) = tower_type.get_bullet(direction, &bullet_assets);
                commands.entity(tower_entity).with_children(|commands| {
                    commands
                        // .spawn(PbrBundle {
                        //     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                        //     material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                        //     transform: Transform::from_xyz(0.0, 0.7, 0.6),
                        //     // .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
                        //     ..default()
                        // })
                        .spawn(SceneBundle {
                            scene: model,
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(2.0, TimerMode::Once),
                        })
                        .insert(bullet)
                        .insert(Name::new("Bullet"));
                });
            }
        }
    }
}

pub fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    tower_type: TowerType,
) -> Entity {
    let (tower_scene, tower) = tower_type.get_tower(assets);
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            position,
        )))
        .insert(Name::new(format!("{tower_type:?} Tower")))
        .insert(tower_type)
        .insert(tower)
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: tower_scene,
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..default()
            });
        })
        .id()
}

#[derive(Component, Reflect, Default)]
pub struct TowerButtonState {
    pub cost: u32,
    pub affordable: bool,
}

fn grey_tower_buttons(
    mut buttons: Query<(&mut BackgroundColor, &mut TowerButtonState)>,
    player: Query<&Player>,
) {
    let player = player.single();

    for (mut tint, mut state) in &mut buttons {
        if player.money >= state.cost {
            state.affordable = true;
            *tint = Color::WHITE.into();
        } else {
            state.affordable = false;
            *tint = Color::DARK_GRAY.into();
        }
    }
}
