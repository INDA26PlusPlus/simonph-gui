use bevy::prelude::*;
use bevy::ui_widgets::{observe, Activate, Button};
use super::GameState;
pub struct MenuPlugin;
impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu);
    }
}

pub fn setup_main_menu(mut commands:Commands){
    commands.spawn((
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items:AlignItems::Center,
            ..default()
        },
        MainMenuEntity,
        children![
            (
                Button,
                Node{
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                children![
                    (
                        Text::new("Start Game"),
                        TextFont {
                            font_size: FontSize::Vh(20.0),
                            ..default()
                        },
                        TextColor(Color::WHITE)
                    )
                ],
                observe(|_:On<Activate>,mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::PlayingGame);
                }),
            )
        ]
    ));
}

#[derive(Component)]
pub struct MainMenuEntity;

pub fn cleanup_main_menu(mut commands:Commands, query:Query<Entity,With<MainMenuEntity>>){
    for entity in query{
        commands.entity(entity).despawn();
    }
}