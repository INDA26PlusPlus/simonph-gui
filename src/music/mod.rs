use bevy::prelude::*;

use super::GameState;
use super::chessgame::states::BoardState;
pub struct MusicPlugin;
impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadingResources), load_music)
        .add_systems(OnEnter(GameState::MainMenu),play_main_menu)
        .add_systems(OnEnter(BoardState::Startup),play_battle_theme)
        .add_systems(OnEnter(BoardState::GameOver),play_checkmate);
    }
}
#[derive(Resource)]
pub struct MusicHandler{
    pub main_menu:Handle<AudioSource>,
    pub playing:Handle<AudioSource>,
    pub checkmate:Handle<AudioSource>
}
pub fn load_music(mut commands:Commands, asset_server: Res<AssetServer>){
    commands.insert_resource(MusicHandler{
        main_menu:asset_server.load("music/checkmate.ogg"),
        playing: asset_server.load("music/theme.ogg"),
        checkmate: asset_server.load("music/checkmate.ogg")
    });
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/checkmate.ogg")),
        PlaybackSettings::LOOP,
        Music,
    ));
}

#[derive(Component)]
pub struct Music;
pub fn play_main_menu(query:Query<Entity, With<Music>>, music_handler: Res<MusicHandler>,mut commands:Commands){
    for entity in query{
        commands.entity(entity).despawn();
    }
    commands.spawn((
        Music,
        AudioPlayer::new(music_handler.main_menu.clone()),
        PlaybackSettings::LOOP
    ));
}
pub fn play_battle_theme(query:Query<Entity, With<Music>>, music_handler: Res<MusicHandler>,mut commands:Commands){
    for entity in query{
        commands.entity(entity).despawn();
    }
    commands.spawn((
        Music,
        AudioPlayer::new(music_handler.playing.clone()),
        PlaybackSettings::LOOP
    ));
}

pub fn play_checkmate(query:Query<Entity, With<Music>>, music_handler: Res<MusicHandler>,mut commands:Commands){
    for entity in query{
        commands.entity(entity).despawn();
    }
    commands.spawn((
        Music,
        AudioPlayer::new(music_handler.checkmate.clone()),
        PlaybackSettings::LOOP
    ));
}