use bevy::prelude::*;
use super::movelistener::SquareClicked;
#[derive(Component)]
pub struct Square{}
#[derive(Component)]
pub struct BoardPosition{
    x:usize,
    y:usize
}
pub fn makeboard(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<ColorMaterial>>){
    let gridw = 500.0;
    let tilew = gridw/8.0;
    let tile_mesh = meshes.add(Rectangle::new(tilew,tilew));
    let white_colour = materials.add(Color::srgb_u8(250, 250, 250));
    let black_colour = materials.add(Color::srgb_u8(0,0,0));
    for i in 0..8{
        for j in 0..8{
            let posx = -tilew*3.5 + tilew*(i as f32);
            let posy = -tilew*3.5 + tilew*(j as f32);
            let tilecolour = match (i+j)%2{
                1 => white_colour.clone(),
                0 => black_colour.clone(),
                _ => panic!("aah"),
            };
            commands.spawn((
                Square{},
                BoardPosition{x:i,y:j},
                Mesh2d(tile_mesh.clone()),
                MeshMaterial2d(tilecolour),
                Transform::from_xyz(posx,posy,0.0),
            ));
        }
    }
}
pub fn click_square(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    squares: Query<(&BoardPosition, &GlobalTransform),With<Square>>,
    mut commands:Commands
) {
    if !mouse.just_pressed(MouseButton::Left){
        return;
    }
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
    for (square, transform) in &squares{
        let half = 500.0/16.0;
        let square_position = transform.translation().truncate();
        if square_position.x - half < world_position.x 
        && world_position.x < square_position.x + half
        && square_position.y - half < world_position.y 
        && world_position.y < square_position.y + half{
            println!("clicked {} {}", square.x, square.y);
            commands.trigger(SquareClicked{square:(square.x,square.y)});
        }
    }
}