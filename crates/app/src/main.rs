mod grid;

use bevy::prelude::*;
use grid::GridPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GridPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands : Commands){
    commands.spawn(Camera2d);

    commands.spawn(Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::splat(5.0)),
        ..default()
    });
}
