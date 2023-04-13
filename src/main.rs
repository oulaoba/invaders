use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Invaders".to_owned(),
                resolution: (598., 676.).into(),
                ..Window::default()
            }),
            ..WindowPlugin::default()
        }))
        .add_system(bevy::window::close_on_esc)
        .run();
}
