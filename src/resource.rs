use bevy::{
    prelude::{AudioSource, Handle, Image, Resource, States},
    sprite::TextureAtlas,
    text::Font,
};

/// 游戏窗口大小资源
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

/// 游戏图像资源
#[derive(Resource)]
pub struct GameTextures {
    pub background: Handle<Image>,
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
    pub font: Handle<Font>,
}

/// 敌人最大数量
#[derive(Resource)]
pub struct MaxEnemy(pub u32);

/// 玩家状态
#[derive(Resource)]
pub struct PlayerState {
    pub on: bool,
    pub last_shot: f64,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
        }
    }
}

impl PlayerState {
    /// 被命中
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }
    /// 重生
    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}

#[derive(Resource)]
pub struct GameAudio {
    pub enemy_explosion: Handle<AudioSource>,
    pub player_explosion: Handle<AudioSource>,
    pub player_laser: Handle<AudioSource>,
}

/// 游戏状态    
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// 欢迎
    #[default]
    Welcome,
    /// 游戏中
    InGame,
    /// 暂停
    Paused,
}

/// 游戏数据
#[derive(Resource)]
pub struct GameData {
    score: u32,
}

impl GameData {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    /// 获取当前得分
    pub fn get_score(&self) -> u32 {
        self.score
    }

    /// 增加得分
    pub fn add_score(&mut self) {
        self.score += 1;
    }

    /// 增加得分
    pub fn reset_score(&mut self) {
        self.score = 0;
    }
}
