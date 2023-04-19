use bevy::{
    prelude::{
        Color, Commands, Entity, Input, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs,
        KeyCode, NextState, OnEnter, OnExit, OnUpdate, Plugin, Query, Res, ResMut, Transform, Vec3,
        With,
    },
    text::{Text, Text2dBundle, TextAlignment, TextSection, TextStyle},
    time::Time,
};

use crate::{
    components::{PausedText, WelcomeText},
    resource::{GameState, GameTextures},
};

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // 在 CoreSet::StateTransitions 期间,当 AppState::Menu 时，该函数执行，
            //当退出该状态进入下一个状态时，会先执行当前状态的退出函数，再执行下个状态的函数
            // OnEnter 进入时执行、OnUpdate 期间内每帧执行、OnExit 退出时执行
            .add_system(welcome_system.in_schedule(OnEnter(GameState::Welcome)))
            // CoreSet::Update 期间 主函数中的 on_update 将会检查 State 资源的值，并判断是否应该运行
            .add_systems(
                (welcome_input_system, welcome_text_scale_system)
                    .in_set(OnUpdate(GameState::Welcome)),
            )
            .add_system(welcome_exit_system.in_schedule(OnExit(GameState::Welcome)))
            // Paused 状态下执行的函数
            .add_system(paused_system.in_schedule(OnEnter(GameState::Paused)))
            .add_system(paused_input_system.in_set(OnUpdate(GameState::Paused)))
            .add_system(paused_exit_system.in_schedule(OnExit(GameState::Paused)));
    }
}

/// 欢迎状态下运行的系统
pub fn welcome_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // 字体引入
    let font = game_textures.font.clone();
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 46.,
        color: Color::BLUE,
    };
    let text_alignment = TextAlignment::Center;

    let text = Text {
        sections: vec![
            TextSection::new("PRESS ", text_style.clone()),
            TextSection::new(
                " ENTER ",
                TextStyle {
                    color: Color::RED,
                    ..text_style.clone()
                },
            ),
            TextSection::new("START GAME !\r\n", text_style.clone()),
            TextSection::new("PRESS ", text_style.clone()),
            TextSection::new(
                " P ",
                TextStyle {
                    color: Color::RED,
                    ..text_style.clone()
                },
            ),
            TextSection::new("TO PAUSED GAME !", text_style.clone()),
        ],
        ..Default::default()
    }
    .with_alignment(text_alignment);
    commands.spawn((
        Text2dBundle {
            text,
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -20.,
                    z: 11.,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        WelcomeText,
    ));
}

/// 欢迎状态状态下的键盘监听系统
pub fn welcome_input_system(kb: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if kb.just_pressed(KeyCode::Return) {
        next_state.set(GameState::InGame);
    }
}

/// 欢迎状态字体变化系统
pub fn welcome_text_scale_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<WelcomeText>)>,
) {
    for mut transform in &mut query {
        transform.scale = Vec3::splat(time.elapsed_seconds().sin() / 4. + 0.9);
    }
}

/// 退出欢迎状态时执行的系统
pub fn welcome_exit_system(
    mut commands: Commands,
    query: Query<Entity, (With<Text>, With<WelcomeText>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// 暂停状态下运行的系统
pub fn paused_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // 字体引入
    let font = game_textures.font.clone();
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 46.,
        color: Color::BLUE,
    };
    let text_alignment = TextAlignment::Center;

    let text = Text {
        sections: vec![
            TextSection::new("GAME PAUSED!\r\nPRESSED", text_style.clone()),
            TextSection::new(
                " R ",
                TextStyle {
                    color: Color::RED,
                    ..text_style.clone()
                },
            ),
            TextSection::new("RETURN GAME!", text_style.clone()),
        ],
        ..Default::default()
    }
    .with_alignment(text_alignment);
    commands.spawn((
        Text2dBundle {
            text,
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -20.,
                    z: 11.,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        PausedText,
    ));
}

/// 暂停状态状态下的键盘监听系统
pub fn paused_input_system(kb: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if kb.pressed(KeyCode::R) {
        next_state.set(GameState::InGame);
    }
}

/// 退出暂停状态时执行的系统
pub fn paused_exit_system(
    mut commands: Commands,
    query: Query<Entity, (With<Text>, With<PausedText>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
