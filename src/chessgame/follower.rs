use bevy::prelude::*;
#[derive(Component)]
pub struct FollowMouse;
pub fn follow_mouse(
    query: Query<&mut Transform,With<FollowMouse>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
){
    let window = windows.single().unwrap();
    let cursor_position = match window.cursor_position(){
        Some(v) => v,
        None => return,
    };
    let (camera, camera_position) = camera.single().unwrap();

    let world_position = match camera.viewport_to_world_2d(camera_position, cursor_position){
        Ok(v) => v,
        Err(_) => return
    };
    for mut transform in query{
        *transform=Transform::from_xyz(world_position.x, world_position.y, transform.translation.z);
    }
}
