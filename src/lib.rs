use bevy::prelude::*;
use bevy_mod_picking::Selection;

pub mod bullet;
pub mod main_menu;
pub mod player;
pub mod target;
pub mod tower;

pub use bullet::*;
pub use main_menu::*;
pub use player::*;
pub use target::*;
pub use tower::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

pub const CUBE_COLOUR: Color = Color::rgb(0.67, 0.84, 0.92);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    GamePlay,
}

#[derive(Resource, Default)]
pub struct GameAssets {
    // pub bullet_scene: Handle<Scene>,
    pub tower_base_scene: Handle<Scene>,
    pub tomato_tower_scene: Handle<Scene>,
    pub tomato_scene: Handle<Scene>,
    pub potato_tower_scene: Handle<Scene>,
    pub potato_scene: Handle<Scene>,
    pub cabbage_tower_scene: Handle<Scene>,
    pub cabbage_scene: Handle<Scene>,
    pub target_scene: Handle<Scene>,
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        // bullet_scene: assets.load("Bullet.glb#Scene0"),
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        potato_tower_scene: assets.load("PotatoTower.glb#Scene0"),
        potato_scene: assets.load("Potato.glb#Scene0"),
        cabbage_tower_scene: assets.load("CabbageTower.glb#Scene0"),
        cabbage_scene: assets.load("Cabbage.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

pub fn camera_controls(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let mut up = camera.up();
    up = up.normalize();

    let speed = 3.0;
    let rotate_speed = 1.0;

    if keyboard.pressed(KeyCode::Space) {
        camera.translation += up * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::LShift) {
        camera.translation -= up * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_y(rotate_speed * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::E) {
        camera.rotate_y(-rotate_speed * time.delta_seconds());
    }
}

// pub fn what_is_selected(selection: Query<(&Name, &Selection)>) {
//     for (name, selection) in &selection {
//         if selection.selected() {
//             info!("{}", name);
//         }
//     }
// }

#[derive(Component, Reflect)]
pub struct TowerUIRoot;

#[derive(Component, Clone, Copy, Debug)]
pub enum TowerType {
    Tomato,
    Potato,
    Cabbage,
}

impl TowerType {
    fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        use TowerType::*;
        match self {
            Tomato => (
                assets.tomato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                    range: 4.5,
                },
            ),
            Potato => (
                assets.potato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                    range: 4.5,
                },
            ),
            Cabbage => (
                assets.cabbage_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.8, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                    range: 4.5,
                },
            ),
        }
    }

    fn get_bullet(&self, direction: Vec3, assets: &GameAssets) -> (Handle<Scene>, Bullet) {
        use TowerType::*;
        match self {
            Tomato => (
                assets.tomato_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                },
            ),
            Potato => (
                assets.potato_scene.clone(),
                Bullet {
                    direction,
                    speed: 6.5,
                },
            ),
            Cabbage => (
                assets.cabbage_scene.clone(),
                Bullet {
                    direction,
                    speed: 1.5,
                },
            ),
        }
    }
}

pub fn create_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let towers = [TowerType::Tomato, TowerType::Potato, TowerType::Cabbage];

    let costs = [50, 80, 110];

    let button_icons: [Handle<Image>; 3] = [
        asset_server.load("images/tomato_tower.png"),
        asset_server.load("images/potato_tower.png"),
        asset_server.load("images/cabbage_tower.png"),
    ];

    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            // background_color: Color::RED.into(),
            ..default()
        })
        .insert(TowerUIRoot)
        .with_children(|commands| {
            for i in 0..=2 {
                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(
                                Val::Percent(15.0 * HEIGHT / WIDTH),
                                Val::Percent(15.0),
                            ),
                            align_self: AlignSelf::FlexEnd,
                            margin: UiRect::all(Val::Percent(2.0)),
                            ..default()
                        },
                        image: button_icons[i].clone().into(),
                        ..default()
                    })
                    .insert(TowerButtonState {
                        cost: costs[i],
                        affordable: false,
                    })
                    .insert(towers[i]);
            }
        });
}

pub fn create_ui_on_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // TODO: Implement on change detection
    selections: Query<&Selection>,
    root: Query<Entity, With<TowerUIRoot>>,
) {
    let at_least_one_selected = selections.iter().any(|selection| selection.selected());

    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        }
        Err(bevy::ecs::query::QuerySingleError::NoEntities(..)) => {
            if at_least_one_selected {
                create_ui(&mut commands, &asset_server);
            }
        }
        _ => unreachable!("Too many UI Tower Roots!"),
    }
}

pub fn tower_button_clicked(
    interactions: Query<(&Interaction, &TowerType, &TowerButtonState), Changed<Interaction>>,
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    mut player: Query<&mut Player>,
    assets: Res<GameAssets>,
) {
    let mut player = player.single_mut();
    for (interaction, tower_type, button_state) in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selection {
                if selection.selected() && player.money >= button_state.cost {
                    player.money -= button_state.cost;

                    commands.entity(entity).despawn_recursive();

                    spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                }
            }
        }
    }
}
