use crate::*;

#[derive(Component, Reflect, Default)]
pub struct Player {
    pub money: u32,
    pub health: u32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(
                (spawn_player, spawn_gameplay_ui).in_schedule(OnEnter(GameState::GamePlay)),
            )
            .add_systems(
                (give_money_on_kill, hurt_player, update_player_ui)
                    .in_set(OnUpdate(GameState::GamePlay)),
            );
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {
            money: 100,
            health: 10,
        },
        Name::new("Player"),
    ));
}

fn give_money_on_kill(
    mut player: Query<&mut Player>,
    mut death_events: EventReader<TargetDeathEvent>,
) {
    let mut player = player.single_mut();
    for _event in death_events.iter() {
        player.money += 10;
    }
}

fn hurt_player(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
    path: Res<TargetPath>,
    mut player: Query<&mut Player>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (entity, target) in &targets {
        if target.path_index >= path.waypoints.len() {
            commands.entity(entity).despawn_recursive();

            audio.play(asset_server.load("damage.wav"));

            let mut player = player.single_mut();
            if player.health > 0 {
                player.health -= 1;
            }

            if player.health == 0 {
                info!("GAME OVER!");
            }
        }
    }
}

#[derive(Component, Reflect)]
pub struct GamePlayUIRoot;

#[derive(Component, Reflect)]
pub struct MoneyUI;

#[derive(Component, Reflect)]
pub struct HealthUI;

fn spawn_gameplay_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .insert(GamePlayUIRoot)
        .with_children(|commands| {
            commands.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    align_self: AlignSelf::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            });
        })
        .with_children(|commands| {
            commands
                .spawn(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Percent(1.2)),
                        ..default()
                    },
                    text: Text::from_section(
                        "Player Money: XX",
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(MoneyUI);

            commands
                .spawn(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Percent(1.2)),
                        ..default()
                    },
                    text: Text::from_section(
                        "Player Health: XX",
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                })
                .insert(HealthUI);
        });
}

fn update_player_ui(
    player: Query<&Player>,
    mut money_ui: Query<&mut Text, (With<MoneyUI>, Without<HealthUI>)>,
    mut health_ui: Query<&mut Text, With<HealthUI>>,
) {
    let player = player.single();
    let mut money = money_ui.single_mut();
    let mut health = health_ui.single_mut();

    *money = Text::from_section(
        format!("Money: {}", player.money),
        money.sections[0].style.clone(),
    );

    *health = Text::from_section(
        format!("Health: {}", player.health),
        health.sections[0].style.clone(),
    );
}
