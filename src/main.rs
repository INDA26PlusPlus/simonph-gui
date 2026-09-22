use bevy::prelude::*;
mod chessgame;
mod menu;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(chessgame::ChessGamePlugin)
        .add_plugins(menu::MenuPlugin)
        .insert_resource(ClearColor(Color::srgb_u8(150, 252, 116)))
        .add_systems(Startup, setup)
        .add_systems(Update, tempmenu)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}


pub fn tempmenu(keyboard:Res<ButtonInput<KeyCode>>,mut commands: Commands){
    if keyboard.just_pressed(KeyCode::Enter){
        commands.trigger(chessgame::MakeBoard{});
    }
}




