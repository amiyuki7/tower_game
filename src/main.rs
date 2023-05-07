use bevy::{pbr::NotShadowCaster, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;
use tower_game::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                title: "Tower".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(GameAssets::default())
        .add_startup_system(asset_loading.in_base_set(StartupSet::PreStartup))
        // .add_startup_system(create_ui_on_selection)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_system(camera_controls)
        // .add_system(what_is_selected)
        .add_system(create_ui_on_selection)
        .add_system(tower_button_clicked)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PickingCameraBundle::default());
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    for i in 1..=10 {
        commands
            .spawn(SceneBundle {
                scene: game_assets.target_scene.clone(),
                transform: Transform::from_xyz(-i as f32, 0.2, 1.5),
                ..default()
            })
            .insert(Target { speed: 0.3 })
            .insert(Health { value: 3 })
            .insert(Name::new("Target"));

        // // Row 2
        // commands
        //     .spawn(PbrBundle {
        //         mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
        //         material: materials.add(CUBE_COLOUR.into()),
        //         transform: Transform::from_xyz(-2.0 * i as f32, 1.0, 1.5),
        //         ..default()
        //     })
        //     .insert(Target { speed: 0.35 })
        //     .insert(Health { value: 3 })
        //     .insert(Name::new("Target"));
    }

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 50.0,
                ..default()
            })),
            material: materials.add(Color::SEA_GREEN.into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    let default_collider_colour = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_colour = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    for i in 1..=3 {
        commands
            .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                (i * 2) as f32,
                0.8,
                0.0,
            )))
            .insert(Name::new("Tower Base"))
            .insert(meshes.add(shape::Capsule::default().into()))
            .insert(Highlighting {
                initial: default_collider_colour.clone(),
                hovered: Some(selected_collider_colour.clone()),
                pressed: Some(selected_collider_colour.clone()),
                selected: Some(selected_collider_colour.clone()),
            })
            .insert(default_collider_colour.clone())
            .insert(NotShadowCaster)
            .insert(PickableBundle::default())
            .with_children(|commands| {
                commands.spawn(SceneBundle {
                    scene: game_assets.tower_base_scene.clone(),
                    transform: Transform::from_xyz(0.0, -0.8, 0.0),
                    ..default()
                });
            });
    }

    // commands
    //     .spawn(SpatialBundle::from_transform(Transform::from_xyz(
    //         0.0, 0.8, 0.0,
    //     )))
    //     .insert(Name::new("Tower Base"))
    //     .insert(meshes.add(shape::Capsule::default().into()))
    //     .insert(Highlighting {
    //         initial: default_collider_colour.clone(),
    //         hovered: Some(selected_collider_colour.clone()),
    //         pressed: Some(selected_collider_colour.clone()),
    //         selected: Some(selected_collider_colour),
    //     })
    //     .insert(default_collider_colour)
    //     .insert(NotShadowCaster)
    //     .insert(PickableBundle::default())
    //     .with_children(|commands| {
    //         commands.spawn(SceneBundle {
    //             scene: game_assets.tower_base_scene.clone(),
    //             transform: Transform::from_xyz(0.0, -0.8, 0.0),
    //             ..default()
    //         });
    //     });

    // // .spawn(PbrBundle {
    // //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    // //     material: materials.add(CUBE_COLOUR.into()),
    // //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    // //     ..default()
    // // })
    // .spawn(SceneBundle {
    //     scene: game_assets.tower_base_scene.clone(),
    //     ..default()
    // })
    // // .insert(Tower {
    // //     bullet_offset: Vec3::new(0.0, 0.2, 0.5),
    // //     shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    // // })
    // .insert(Name::new("Tower"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
