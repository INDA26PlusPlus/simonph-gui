use bevy::prelude::*;
use crate::chessgame::movevalidator::MetaBoard;

use super::event::BoardUpdated;
use super::movelistener::SquareClicked;
use super::piece::PieceComponent;
use super::boardconstants::*;
use super::states::*;
#[derive(Component)]
pub struct Square{}
#[derive(Component)]
pub struct BoardPosition{
    pub x:usize,
    pub y:usize
}
pub fn get_world_position(x:usize, y:usize) -> (f32,f32){
    let posx = -TILE_SIZE*3.5 + TILE_SIZE*(x as f32);
    let posy = -TILE_SIZE*3.5 + TILE_SIZE*(y as f32);
    (posx,posy)
}
pub fn makeboard(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meta_board: ResMut<MetaBoard>
)
{
    *meta_board = MetaBoard::default();
    let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE,TILE_SIZE));
    
    let border_colour = materials.add(Color::srgb_u8(93, 44, 40));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(BOARD_SIZE + BORDER_SIZE,BOARD_SIZE + BORDER_SIZE))),
        Transform::from_xyz(0.0,0.0,-1.0),
        MeshMaterial2d(border_colour),
    ));
    for i in 0..8{
        for j in 0..8{
            let (posx,posy) = get_world_position(i,j);
            let tilecolour = match (i+j)%2{
                1 => WHITE_COLOUR.clone(),
                0 => BLACK_COLOUR.clone(),
                _ => panic!("aah"),
            };
            let tilematerial = materials.add(tilecolour);
            commands.spawn((
                Square{},
                BoardPosition{x:i,y:j},
                Mesh2d(tile_mesh.clone()),
                MeshMaterial2d(tilematerial),
                Transform::from_xyz(posx,posy,0.0),
            ));
            let piecesprite = Sprite::default();
            commands.spawn((
                PieceComponent{},
                BoardPosition{x:i,y:j},
                piecesprite,
                Transform::from_xyz(posx, posy, 1.0),
            ));
        }
    }
    commands.trigger(BoardUpdated{});
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
#[derive(Component)]
pub struct SquareHighlight{}

pub fn highlight_square(move_state: Res<State<MoveState>>, mut commands: Commands, query:Query<(Entity,&BoardPosition),With<Square>>){
    let square = match *move_state.get(){
        MoveState::None => return,
        MoveState::ActiveSquare(x,y ) => (x,y)
    };
    for (entity, pos) in query{
        if pos.x != square.0 || pos.y != square.1{
            continue;
        }
        commands.entity(entity).insert(SquareHighlight{});
    }
}
pub fn added_active(
    query: Query<&MeshMaterial2d<ColorMaterial>, Added<SquareHighlight>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for material in &query {
        if let Some(mut material) = materials.get_mut(&material.0) {
            material.color = Color::srgb(1.0, 1.0, 0.0);
        }
    }
}
pub fn highlight_legal_moves(move_state: Res<State<MoveState>>, 
    mut commands: Commands, 
    query:Query<(Entity,&BoardPosition),With<Square>>,
    meta_board:Res<MetaBoard>)

{
    println!("gaah");
    let square = match *move_state.get(){
        MoveState::None => return,
        MoveState::ActiveSquare(x,y)=> (x,y),
    };
    let legal_moves = meta_board.get_legal_moves(square);
    for x in &legal_moves{
        println!("{} {}",x.0,x.1);
    }
    for (entity, pos) in query{
        if legal_moves.contains(&(pos.x,pos.y)){
            commands.entity(entity).insert(SquareHighlight{});
        }
        
    }
}
pub fn remove_active(
    query: Query<(&MeshMaterial2d<ColorMaterial>,&BoardPosition,Entity),With<SquareHighlight>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
){
    for (material,pos,entity) in &query{
        let tilecolour = match (pos.x+pos.y)%2{
            1 => WHITE_COLOUR.clone(),
            0 => BLACK_COLOUR.clone(),
            _ => panic!("aah"),
        };
        if let Some(mut material) = materials.get_mut(&material.0) {
            material.color = tilecolour;
        }
        commands.entity(entity).remove::<SquareHighlight>();
    }
}