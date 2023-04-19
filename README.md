# Rust 实现的飞机游戏

## 简介
一个使用 bevy 引擎制作的飞机游戏。

原[视频教程](https://www.youtube.com/watch?v=j7qHwb7geIM)地址,[github 地址](https://github.com/jeremychone-channel/rust-invaders)。

因为 bevy 已经升级到 0.10.1 了，所以重新做一遍。顺带手出个教程。

下面是做的部分变动：

- 将激光以及玩家的移动模块进行了拆分。
- 新增了背景图片。
- 新增了游戏状态管理 Welcome/InGame/Paused。
- 新增了声音播放模块。
- 新增了游戏记分板。

通过左右方向键进行控制，使用空格发射激光。

按 P 暂停游戏，按 S 恢复游戏。

## 代码结构
```
·
├── assets/
│   ├──audios/
│   ├──fonts/
│   └──images/
├── src/
│   ├──enemy/
│   │  ├── formation.rs
│   │  └── mod.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── main.rs
│   ├── player.rs
│   ├── resource.rs
│   └── state.rs
├── Cargo.lock
└── Cargo.toml
```

- assets/audios 声音资源文件。
- assets/fonts 字体资源文件。
- assets/images 图片资源文件。
- enemy/formation.rs 敌人阵型系统的实现。
- enemy/mod.rs 敌人插件，生成、移动、攻击的实现。
- components.rs 负责游戏的逻辑、控制、等内容。
- constants.rs 负责存储游戏中用到的常量。
- main.rs 负责游戏的逻辑、控制、等内容。
- player.rs 玩家角色插件，生成、移动、攻击、键盘处理的实现。
- resource.rs 游戏资源定义。
- state.rs 游戏组件定义。

## about me 
目前失业，在家学习 rust 。

我的 [bilibili](https://space.bilibili.com/259260787),我的 [博客园](https://www.cnblogs.com/SantiagoZhang/)。
