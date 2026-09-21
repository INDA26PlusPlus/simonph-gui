use bevy::prelude::*;
mod chessgame;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(chessgame::ChessGamePlugin)
        .insert_resource(ClearColor(Color::srgb_u8(150, 252, 116)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}






