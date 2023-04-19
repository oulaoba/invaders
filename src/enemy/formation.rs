use bevy::prelude::{Component, Resource};
use rand::{thread_rng, Rng};

use crate::{WinSize, BASE_SPEED, FORMATION_MEMBER_MAX};

/// 敌人阵型
#[derive(Component, Clone)]
pub struct Formation {
    /// 启始位置
    pub start: (f32, f32),
    /// 半径
    pub radius: (f32, f32),
    /// 原点
    pub pivot: (f32, f32),
    /// 速度
    pub speed: f32,
    /// 角度
    pub angle: f32,
}

/// 阵型资源
#[derive(Resource, Default)]
pub struct FormationMaker {
    /// 当前阵型
    current_template: Option<Formation>,
    /// 当前数量
    current_members: u32,
}

impl FormationMaker {
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        match (
            &self.current_template,
            self.current_members >= FORMATION_MEMBER_MAX,
        ) {
            // 当前阵型还有空位 直接加入
            (Some(template), false) => {
                self.current_members += 1;
                template.clone()
            }
            // 当前阵型没有空位，或还没有阵型，需要创建新的阵型
            _ => {
                let mut rng = thread_rng();

                // 生成 起点坐标
                let w_spawn = win_size.w / 2. + 100.;
                let h_spawn = win_size.h / 2. + 100.;
                let x = if rng.gen_bool(0.5) { w_spawn } else { -w_spawn };
                let y = rng.gen_range(-h_spawn..h_spawn);
                let start = (x, y);

                // 生成原点坐标
                let w_spawn = win_size.w / 4.;
                let h_spawn = win_size.h / 3. + 50.;
                let pivot = (
                    rng.gen_range(-w_spawn..w_spawn),
                    rng.gen_range(0. ..h_spawn),
                );

                // 生成半径
                let radius = (rng.gen_range(80. ..150.), 100.);

                // 计算初始角度
                let angle = (y - pivot.1).atan2(x - pivot.0);

                // 速度
                let speed = BASE_SPEED;

                let formation = Formation {
                    start,
                    pivot,
                    radius,
                    angle,
                    speed,
                };

                self.current_template = Some(formation.clone());
                self.current_members = 1;
                formation
            }
        }
    }
}
