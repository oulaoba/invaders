use std::{f32::consts::PI, time::Duration};

use crate::{
    components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity},
    resource::GameState,
    GameTextures, MaxEnemy, WinSize, ENEMY_LASER_SIZE, ENEMY_SIZE, MAX_ENEMY, SPRITE_SCALE,
    TIME_STEP,
};

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::{thread_rng, Rng};

use self::formation::{Formation, FormationMaker};

mod formation;

#[derive(Component)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // 间隔执行
        app.insert_resource(FormationMaker::default())
            .add_system(
                enemy_spawn_system
                    .run_if(on_timer(Duration::from_secs_f32(0.5)))
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(
                enemy_fire_system
                    .run_if(enemy_fire_criteria)
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(enemy_movement_system.in_set(OnUpdate(GameState::InGame)));
    }
}

/// 敌人生成系统
fn enemy_spawn_system(
    mut commands: Commands,
    mut max_enemy: ResMut<MaxEnemy>,
    mut formation_maker: ResMut<FormationMaker>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // 如果当前的敌人数量大于等于最大敌人数量，则不再产生新的敌人
    if max_enemy.0 >= MAX_ENEMY {
        return;
    }

    // 随机生成
    // let mut rng = thread_rng();
    // let w_span = win_size.w / 2. - 100.;
    // let h_span = win_size.h / 2. - 100.;
    // let x = rng.gen_range(-w_span..w_span);
    // let y = rng.gen_range(-h_span..h_span);

    // 使用 阵型
    let formation = formation_maker.make(&win_size);
    let (x, y) = formation.start;

    commands
        .spawn(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                // 坐标
                translation: Vec3::new(x, y, 10.),
                // 缩放
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                // 旋转
                rotation: Quat::IDENTITY,
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(formation)
        .insert(SpriteSize::from(ENEMY_SIZE));
    max_enemy.0 += 1;
}

/// 敌人射击系统
fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Enemy>>,
) {
    for &enemy_tf in query.iter() {
        let (x, y) = (enemy_tf.translation.x, enemy_tf.translation.y);

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 1.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    rotation: Quat::from_rotation_x(PI),
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity::new(0., -1.));
    }
}

/// 是否发射攻击
fn enemy_fire_criteria() -> bool {
    if thread_rng().gen_bool(1. / 60.) {
        true
    } else {
        false
    }
}

/// 敌人移动系统
///
/// 两点间的距离公式 $|AB|=\sqrt{(x_1-x_2)^2+(y_1-y_2)^2}$
fn enemy_movement_system(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>) {
    // 当前时间
    // let now = time.elapsed_seconds();
    for (mut transform, mut formation) in query.iter_mut() {
        // 当前坐标
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);
        // let (x_org, y_org) = formation.start;

        // 单位时间内最大移动距离
        // let max_distance = BASE_SPEED * TIME_STEP;
        let max_distance = formation.speed * TIME_STEP;

        // 方向 1 顺时针 -1 逆时针
        // let dir = -1.;
        let dir = if formation.start.0 < 0. { 1. } else { -1. };
        // 中心点
        // let (x_pivot, y_pivot) = (0., 0.);
        let (x_pivot, y_pivot) = formation.pivot;
        // 半径
        // let (x_radius, y_radius) = (200., 130.);
        let (x_radius, y_radius) = formation.radius;

        // 基于当前时间计算的角度
        // let angel = dir * BASE_SPEED * TIME_STEP * now % 360. / PI;
        let angel = formation.angle
            + dir * formation.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);

        // 计算目标点位
        let x_dst = x_radius * angel.cos() + x_pivot;
        let y_dst = y_radius * angel.sin() + y_pivot;

        // 计算距离
        // 两点间的距离公式 根号下 a.x - b.x
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;

        let distance = (dx * dx + dy * dy).sqrt();
        let distance_radio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };

        // 计算 x y 的最终坐标
        let x = x_org - dx * distance_radio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_radio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        // 图片资源在椭圆上 或接近椭圆时开始加入旋转
        if distance < max_distance * formation.speed / 20. {
            formation.angle = angel;
        }

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}
