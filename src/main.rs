use bevy::prelude::*;
mod chessgame;
mod menu;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(chessgame::ChessGamePlugin)
        .add_plugins(menu::MenuPlugin)
        .insert_resource(ClearColor(Color::srgb_u8(201, 218, 234)))
        .add_systems(Startup,setup)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::LoadingResources),to_main_menu)
        .add_systems(OnEnter(GameState::MainMenu),menu::setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), menu::cleanup_main_menu)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState{
    #[default]
    LoadingResources,
    PlayingGame,
    MainMenu,
}
pub fn to_main_menu(mut next_state: ResMut<NextState<GameState>>){
    next_state.set(GameState::MainMenu);
}




