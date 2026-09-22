use bevy::prelude::*;
mod chessgame;
mod menu;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(chessgame::ChessGamePlugin)
        .add_plugins(menu::MenuPlugin)
        .insert_resource(ClearColor(Color::srgb_u8(150, 252, 116)))
        .add_systems(Startup,setup)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::MainMenu),menu::setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), menu::cleanup_main_menu)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState{
    PlayingGame,
    #[default]
    MainMenu,
}

pub fn tempmenu(keyboard:Res<ButtonInput<KeyCode>>,mut commands: Commands){
    if keyboard.just_pressed(KeyCode::Enter){
        commands.trigger(chessgame::MakeBoard{});
    }
}




