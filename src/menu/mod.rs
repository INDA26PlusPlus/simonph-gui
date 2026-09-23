use super::GameState;
use super::chessgame::states::BoardState;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button, observe};
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(BoardState::GameOver), setup_checkmate_menu)
            .add_systems(OnExit(BoardState::GameOver), cleanup_checkmate_menu)
            .add_systems(OnEnter(GameState::LoadingResources), load_button_assets)
            .add_systems(Update, handle_pixelbutton);
    }
}

pub fn setup_main_menu(mut commands: Commands, button_assets: Res<ButtonAssets>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        MainMenuEntity,
        children![(
            Button,
            Node {
                //padding:UiRect::all(Val::Vh(5.0)),
                ..default()
            },
            Interaction::None,
            ImageNode {
                image: button_assets.button_normal.clone(),
                image_mode: NodeImageMode::Sliced(button_assets.slicer.clone()),
                ..default()
            },
            children![(
                Node {
                    padding: UiRect {
                        left: Val::Vh(5.0),
                        right: Val::Vh(5.0),
                        top: Val::Vh(0.5),
                        bottom: Val::Vh(0.5),
                    },
                    ..default()
                },
                Text::new("Start Game"),
                TextFont {
                    font: FontSource::Handle(button_assets.font.clone()),
                    font_size: FontSize::Vh(20.0),
                    ..default()
                },
                TextColor(Color::WHITE)
            )],
            observe(
                |_: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::PlayingGame);
                }
            ),
        )],
    ));
}

#[derive(Component)]
pub struct MainMenuEntity;

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuEntity>>) {
    println!("cleaning up");
    for entity in query {
        commands.entity(entity).despawn();
    }
}
#[derive(Component)]
pub struct CheckMateMenuEntity;
pub fn setup_checkmate_menu(mut commands: Commands, button_assets: Res<ButtonAssets>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        CheckMateMenuEntity,
        children![
            ((
                Text::new("CHECKMATE"),
                TextFont {
                    font: FontSource::Handle(button_assets.font.clone()),
                    font_size: FontSize::Vh(20.0),
                    ..default()
                },
                TextColor(Color::WHITE)
            )),
            (
                Button,
                Node { ..default() },
                ImageNode {
                    image: button_assets.button_normal.clone(),
                    image_mode: NodeImageMode::Sliced(button_assets.slicer.clone()),
                    ..default()
                },
                Interaction::None,
                children![(
                    Node {
                        padding: UiRect {
                            left: Val::Vh(5.0),
                            right: Val::Vh(5.0),
                            top: Val::Vh(0.5),
                            bottom: Val::Vh(0.5),
                        },
                        ..default()
                    },
                    Text::new("Go to main menu"),
                    TextFont {
                        font: FontSource::Handle(button_assets.font.clone()),
                        font_size: FontSize::Vh(10.0),
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )],
                observe(
                    |_: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::MainMenu);
                    }
                ),
            )
        ],
    ));
}
pub fn cleanup_checkmate_menu(
    mut commands: Commands,
    query: Query<Entity, With<CheckMateMenuEntity>>,
) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
#[derive(Resource)]
pub struct ButtonAssets {
    pub button_normal: Handle<Image>,
    pub button_hover: Handle<Image>,
    pub button_press: Handle<Image>,
    pub slicer: TextureSlicer,
    pub font: Handle<Font>,
}
pub fn load_button_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let slicer = TextureSlicer {
        border: BorderRect::all(5.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 10.0,
    };
    commands.insert_resource(ButtonAssets {
        button_normal: asset_server.load("MenuUI/darkbutton.png"),
        button_hover: asset_server.load("MenuUI/lightbutton.png"),
        button_press: asset_server.load("MenuUI/highlightedbutton.png"),
        slicer,
        font: asset_server.load("MenuUI/PixelPurl.ttf"),
    });
}
pub fn handle_pixelbutton(
    query: Query<(&Interaction, &mut ImageNode), (Changed<Interaction>, With<Button>)>,
    button_assets: Res<ButtonAssets>,
) {
    for (interaction, mut img) in query {
        match *interaction {
            Interaction::Pressed => img.image = button_assets.button_press.clone(),
            Interaction::Hovered => img.image = button_assets.button_hover.clone(),
            Interaction::None => img.image = button_assets.button_normal.clone(),
        }
    }
}
