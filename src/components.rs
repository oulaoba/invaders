use bevy::{
    prelude::{Component, Vec2, Vec3},
    time::{Timer, TimerMode},
};

// 通用控制组件
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// 移动能力组件
#[derive(Component)]
pub struct Movable {
    /// 自动销毁
    pub auto_despawn: bool,
}

/// 玩家组件
#[derive(Component)]
pub struct Player;

/// 玩家信息组件
#[derive(Component)]
pub struct FromPlayer;

/// 敌人组件
#[derive(Component)]
pub struct Enemy;

/// 敌人信息组件
#[derive(Component)]
pub struct FromEnemy;

/// 激光组件
#[derive(Component)]
pub struct Laser;

/// 图片大小组件
#[derive(Component)]
pub struct SpriteSize(pub Vec2);

/// 实现 (f32,f32) 转 SpritSize
impl From<(f32, f32)> for SpriteSize {
    fn from(value: (f32, f32)) -> Self {
        Self(Vec2::new(value.0, value.1))
    }
}

/// 爆炸组件
#[derive(Component)]
pub struct Explosion;

/// 产生爆炸组件
#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

/// 爆炸事件组件
#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Once))
    }
}

/// 分数显示组件
#[derive(Component)]
pub struct DisplayScore;

/// 欢迎组件
#[derive(Component)]
pub struct WelcomeText;

/// 暂停组件
#[derive(Component)]
pub struct PausedText;
