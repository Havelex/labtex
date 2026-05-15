mod gameplay;
mod player;
mod ui;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.01, 0.01, 0.02)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Particle Ante Prototype (M1)".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            world::WorldPlugin,
            gameplay::GameplayPlugin,
            player::PlayerPlugin,
            ui::UiPlugin,
        ))
        .run();
}
