use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet};
use components::*;

use constants::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use resource::{GameAudio, GameData, GameState, GameTextures, MaxEnemy, PlayerState, WinSize};
use state::StatePlugin;

mod components;
mod constants;
mod enemy;
mod player;
mod resource;
mod state;

fn main() {
    // add_startup_system 启动生命周期时只运行一次 ，
    // add_system 每帧都会被调用方法
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Invaders".to_owned(),
                resolution: (598., 676.).into(),
                position: WindowPosition::At(IVec2::new(2282, 0)),
                ..Window::default()
            }),
            ..WindowPlugin::default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StatePlugin)
        .add_startup_system(setup_system)
        // InGame 状态下执行的函数
        .add_systems(
            (
                laser_movable_system,
                player_laser_hit_enemy_system,
                explosion_to_spawn_system,
                explosion_animation_system,
                enemy_laser_hit_player_system,
                score_display_update_system,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        // 启动 esc 键退出程序
        .add_system(bevy::window::close_on_esc)
        .run();
}

/// 资源加载
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: Query<&mut Window>,
) {
    // 创建2d镜头
    commands.spawn(Camera2dBundle::default());

    // 获取当前窗口
    let window = windows.single_mut();
    let win_w = window.width();
    let win_h = window.height();

    //  添加 WinSize 资源
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // 创建爆炸动画
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::from(EXPLOSION_SIZE), 4, 4, None, None);
    let explosion = texture_atlases.add(texture_atlas);

    // 添加 GameTextures
    let game_texture = GameTextures {
        background: asset_server.load(BACKGROUND_SPRITE),
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        font: asset_server.load(KENNEY_BLOCK_FONT),
        explosion,
    };

    // 声音资源引入
    let game_audio = GameAudio {
        player_laser: asset_server.load(PLAYER_LASER_AUDIO),
        player_explosion: asset_server.load(PLAYER_EXPLOSION_AUDIO),
        enemy_explosion: asset_server.load(ENEMY_EXPLOSION_AUDIO),
    };

    // 背景图片
    commands.spawn(SpriteBundle {
        texture: game_texture.background.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2 { x: win_w, y: win_h }),
            ..Default::default()
        },
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 0.0)),
        ..Default::default()
    });

    // 字体引入
    let font = game_texture.font.clone();
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 32.,
        color: Color::ANTIQUE_WHITE,
    };
    let text_alignment = TextAlignment::Center;

    // 分数展示控件
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("SCORE:0", text_style).with_alignment(text_alignment),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: win_h / 2. - 20.,
                    z: 11.,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        DisplayScore,
    ));

    let game_data = GameData::new();
    commands.insert_resource(game_data);
    commands.insert_resource(game_audio);
    commands.insert_resource(game_texture);
    commands.insert_resource(MaxEnemy(0));
}

/// 激光移动系统
fn laser_movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Laser>>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        // 移动位置
        let translation = &mut transform.translation;
        translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        translation.y += velocity.y * BASE_SPEED * TIME_STEP;

        // 自动销毁
        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// 敌人激光攻击玩家判定系统
fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    audio_source: Res<GameAudio>,
    audio: Res<Audio>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (laser, laser_tf, laser_size) in laser_query.into_iter() {
            let laser_scale = Vec2::from(laser_tf.scale.xy());

            let collision = collide(
                player_tf.translation,
                player_size.0 * player_scale,
                laser_tf.translation,
                laser_size.0 * laser_scale,
            );

            if let Some(_) = collision {
                // 播放音乐
                audio.play(audio_source.player_explosion.clone());
                // 重置分数
                game_data.reset_score();
                next_state.set(GameState::Welcome);
                // 销毁角色
                commands.entity(player_entity).despawn();
                // 记录被命中的时刻
                player_state.shot(time.elapsed_seconds_f64());
                // 销毁激光
                commands.entity(laser).despawn();
                // 产生爆炸动画
                commands.spawn(ExplosionToSpawn(player_tf.translation.clone()));
                break;
            }
        }
    }
}

/// 玩家攻击敌人判定系统
fn player_laser_hit_enemy_system(
    mut commands: Commands,
    audio_source: Res<GameAudio>,
    audio: Res<Audio>,
    mut max_enemy: ResMut<MaxEnemy>,
    mut game_data: ResMut<GameData>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    // 重复删除检测
    let mut despawn_entities: HashSet<Entity> = HashSet::new();
    // 玩家激光
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawn_entities.contains(&laser_entity) {
            continue;
        }

        // 玩家激光的坐标
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // 敌人
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawn_entities.contains(&enemy_entity) || despawn_entities.contains(&laser_entity)
            {
                continue;
            }

            // 敌人坐标
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // collide 定义两个元素的碰撞，a 点坐标，a 的大小，b 点坐标，b 的大小,如果未发生碰撞返回 None
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // 碰撞检测
            if let Some(_) = collision {
                // 敌人数量 -1
                if max_enemy.0 != 0 {
                    max_enemy.0 -= 1;
                }
                game_data.add_score();

                audio.play(audio_source.enemy_explosion.clone());
                // 销毁敌人
                commands.entity(enemy_entity).despawn();
                despawn_entities.insert(enemy_entity);
                // 销毁激光
                commands.entity(laser_entity).despawn();
                despawn_entities.insert(laser_entity);

                // 播放爆炸动画
                commands.spawn(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

/// 爆炸画面生成系统
fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commands.entity(explosion_spawn_entity).despawn();
    }
}

/// 爆炸动画系统
fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut texture_atlas_sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            texture_atlas_sprite.index += 1;
            if texture_atlas_sprite.index >= EXPLOSION_ANIMATION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// 分数更新系统
fn score_display_update_system(
    game_data: Res<GameData>,
    mut query: Query<&mut Text, With<DisplayScore>>,
) {
    for mut text in &mut query {
        let new_str: String = format!("SCORE:{}", game_data.get_score());
        text.sections[0].value = new_str;
    }
}
