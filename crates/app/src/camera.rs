use bevy::input::mouse::{MouseScrollUnit,MouseWheel};
use bevy::prelude::*;



pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement);
        app.add_systems(Update, camera_zoom);
    }
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform , With<Camera>> , 
    time: Res<Time>,
){
    // let mut transform = camera_query.single_mut();

    let mut direction = Vec3::ZERO;


    if keyboard.pressed(KeyCode::KeyW){
        direction.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS){
        direction.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA){
        direction.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD){
        direction.x += 1.0;
    }

    let speed = 500.0;
    
    for mut transform in camera_query.iter_mut(){
        transform.translation += direction.normalize_or_zero() * speed * time.delta_secs();
    }
}


fn camera_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
){
    // let mut projection = camera_query.single_mut();

    for event in scroll_events.read() {
        let zoom_amount = match event.unit {
            MouseScrollUnit::Line => event.y * 0.1,
            MouseScrollUnit::Pixel => event.y * 0.001,
        };

        for mut projection in camera_query.iter_mut(){
            projection.scale += Vec3::splat(zoom_amount);
            projection.scale = projection.scale.max(Vec3::splat(0.1)).min(Vec3::splat(10.0));
        }
    }
    
}
