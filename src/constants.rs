/// 游戏背景图片路径
pub const BACKGROUND_SPRITE: &str = "images/planet05.png";

/// 玩家图片路径
pub const PLAYER_SPRITE: &str = "images/player_a_01.png";
/// 玩家大小
pub const PLAYER_SIZE: (f32, f32) = (144., 75.);
/// 玩家攻击图片路径
pub const PLAYER_LASER_SPRITE: &str = "images/laser_a_01.png";
/// 玩家攻击图片大小
pub const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

/// 敌人图片路径
pub const ENEMY_SPRITE: &str = "images/enemy_a_01.png";
/// 敌人大小
pub const ENEMY_SIZE: (f32, f32) = (144., 75.);
/// 敌人攻击图片路径
pub const ENEMY_LASER_SPRITE: &str = "images/laser_b_01.png";
/// 敌人攻击图片大小
pub const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

/// 爆炸图片路径
pub const EXPLOSION_SHEET: &str = "images/explosion_a_sheet.png";
/// 爆炸图片大小
pub const EXPLOSION_SIZE: (f32, f32) = (64., 64.);
/// 爆炸画面帧数
pub const EXPLOSION_ANIMATION_LEN: usize = 16;

/// 图片缩放比例
pub const SPRITE_SCALE: f32 = 0.5;

/// 步长 (帧数)
pub const TIME_STEP: f32 = 1. / 60.;
/// 基础速度
pub const BASE_SPEED: f32 = 500.;
/// 敌人最大数量
pub const MAX_ENEMY: u32 = 2;
/// 玩家自动重生时间
pub const PLAYER_RESPAWN_DELAY: f64 = 2.;
/// 阵型内敌人最大数量
pub const FORMATION_MEMBER_MAX: u32 = 2;

/// 敌人被摧毁声音
pub const ENEMY_EXPLOSION_AUDIO: &str = "audios/enemy_explosion.ogg";
/// 玩家被摧毁的声音
pub const PLAYER_EXPLOSION_AUDIO: &str = "audios/player_explosion.ogg";
/// 玩家发射激光的声音
pub const PLAYER_LASER_AUDIO: &str = "audios/player_laser.ogg";

/// 字体路径
pub const KENNEY_BLOCK_FONT: &str = "fonts/kenney_blocks.ttf";
