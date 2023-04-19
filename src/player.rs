use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

use crate::{
    components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity},
    resource::GameAudio,
    resource::PlayerState,
    resource::WinSize,
    resource::{GameState, GameTextures},
    BASE_SPEED, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // add_startup_system 应用程序生命周期开始时运行一次
        // StartupSet::PostStartup 在 StartupSet::Startup 后运行一次
        // add_startup_system(player_spawn_system.in_base_set(StartupSet::PostStartup))
        // add_system 每帧都运行 , 可以在函数后通过 run_if 传入 bool 类型的条件进行限制
        app.insert_resource(PlayerState::default())
            .add_system(
                player_spawn_system
                    .run_if(on_timer(Duration::from_secs_f32(0.5)))
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_systems(
                (
                    player_keyboard_event_system,
                    player_movable_system,
                    player_fire_system,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}

/// 玩家角色生成系统
fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let now = time.elapsed_seconds_f64();
    let last_shot = player_state.last_shot;
    if !player_state.on && (player_state.last_shot == -1. || now - PLAYER_RESPAWN_DELAY > last_shot)
    {
        let bottom = -win_size.h / 2.;

        // 创建组件实体，并返回对应的 EntityCommand
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        0.,
                        bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.0,
                        10.,
                    ),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..default()
                },
                ..SpriteBundle::default()
            })
            .insert(Velocity::new(0., 0.))
            .insert(Movable {
                auto_despawn: false,
            })
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Player);

        player_state.spawned();
    }
}

/// 玩家攻击系统
fn player_fire_system(
    mut commands: Commands,
    audio_source: Res<GameAudio>,
    audio: Res<Audio>,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        // just_released 松开按键
        if kb.just_released(KeyCode::Space) {
            audio.play(audio_source.player_laser.clone());
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            // 激光生成闭包 因为这里使用了 commands 生成新的包 所以这里的闭包需要定义为 mut 类型
            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 1.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity::new(0., 1.));
            };
            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}

/// 键盘事件系统
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        // pressed 按下按键
        if kb.pressed(KeyCode::Left) {
            velocity.x = -1.
        } else if kb.pressed(KeyCode::Right) {
            velocity.x = 1.
        } else if kb.just_pressed(KeyCode::P) {
            next_state.set(GameState::Paused);
        } else {
            velocity.x = 0.
        }
    };
}

/// 玩家移动系统
fn player_movable_system(
    win_size: Res<WinSize>,
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    let max_w = win_size.w / 2.;

    for (velocity, mut transform) in query.iter_mut() {
        let distance = velocity.x * BASE_SPEED * TIME_STEP;
        let new_x = transform.translation.x + distance;
        if -max_w <= new_x && new_x <= max_w {
            // 移动位置
            transform.translation.x += distance;
        }
    }
}
