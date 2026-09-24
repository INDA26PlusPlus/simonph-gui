use::bevy::prelude::*;
use super::movevalidator::Piece;
use super::chessassets::PieceAssets;
use super::piece::getposition;
use crate::chessgame::movelistener::MoveListener;
use super::event::PromotionClick;

#[derive(Component)]
pub struct PromotionOption{piece:Piece}

pub fn spawn_single_promotion(commands:&mut Commands, 
    piece_assets:&PieceAssets,
    images: &Assets<Image>, 
    pos:(usize,usize),
    piece:Piece
){
    let img = match piece_assets.get_piece_asset(&piece){
        None => return,
        Some(v) => v,
    };
    let ((w,h),(posx,posy)) = getposition(img.clone(),images,pos.clone());
    commands.spawn((
        PromotionOption{piece},
        Sprite{image:img, custom_size:Some(Vec2::new(w,h)), ..default()},
        Transform::from_xyz(posx, posy, 3.0),
    ));
}

pub fn spawn_promotion(
    mut commands:Commands, 
    piece_assets:Res<PieceAssets>, 
    images: Res<Assets<Image>>,
    move_listener : Res<MoveListener>
){
    let pos = match move_listener.end{
        None => return,
        Some(v) => v,
    };
    let is_white = match pos.1{
        7 => true,
        _ => false,
    };
    let dy:i8 = match pos.1{
        7 => -1,
        _ => 1,
    };
    let piece_arr = [
        Piece::Queen { is_white }, 
        Piece::Knight { is_white },
        Piece::Rook { is_white, has_moved: true }, 
        Piece::Bishop { is_white }
    ];
    for i in 0..4{
        spawn_single_promotion(&mut commands, &piece_assets, &images, (pos.0,(pos.1 as i8 + dy*(i as i8)) as usize), piece_arr[i]);
    }
}
pub fn click_promotion(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    squares: Query<(&GlobalTransform,&PromotionOption)>,
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
    let mut clicked_piece = PromotionClick{piece:Piece::Empty};
    for (transform,prom) in &squares{
        let half = 500.0/16.0;
        let square_position = transform.translation().truncate();
        if square_position.x - half < world_position.x 
        && world_position.x < square_position.x + half
        && square_position.y - half < world_position.y 
        && world_position.y < square_position.y + half{
            clicked_piece.piece = prom.piece;
        }
    }
    commands.trigger(clicked_piece);
}
pub fn remove_promotion(mut commands: Commands, query:Query<Entity,With<PromotionOption>>){
    for entity in query{
        commands.entity(entity).despawn();
    }
}