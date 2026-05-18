use bevy::prelude::*;

pub struct GridPlugin; 

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
    }
}

fn spawn_grid(mut commands : Commands){
    let grid_size = 20; 
    let spacing = 50.0;

    for i in -grid_size..=grid_size {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(1.0, 4000.0)),
                ..default()
            },
            Transform::from_xyz(i as f32 * spacing, 0.0, 0.0),
        ));

        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(4000.0, 1.0)),
                ..default()
            },
            Transform::from_xyz(0.0, i as f32 * spacing, 0.0),
        ));
    }
}