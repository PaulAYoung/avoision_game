
use bevy::{prelude::*};
use crate::game_structs::{GameState, Score, GameEntity};

#[derive(Component)]
pub struct MenuText;

#[derive(Component)]
pub struct ScoreText;
pub struct MenuPlugin;

pub fn get_text_style()->Style {
    Style{
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
        ..default()
    }
}

pub fn init_score(mut commands: Commands, asset_server: Res<AssetServer>, mut score:ResMut<Score>){

    // reset score to 0
    score.reset();

    // spawn score display
    commands
       .spawn_bundle(TextBundle {
            style: Style{
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                format!("Score: {}", score.0.elapsed_secs()),
                TextStyle {
                    font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(ScoreText)
        .insert(GameEntity);
}

pub fn update_score(
        mut query: Query<(&mut Text), With<ScoreText>>,
        time:Res<Time>,
        mut score:ResMut<Score>
    ){
    score.tick(time.delta());
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {:.2}", score.elapsed_secs());
}

pub fn init_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    commands
       .spawn_bundle(TextBundle {
            style: get_text_style(),
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Welcome to Avoision!\nPress Space to Start",
                TextStyle {
                    font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(MenuText);
}

pub fn start_game(
    mut game_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::Space){
        game_state.set(GameState::InGame).unwrap();
    }
}

pub fn exit_menu(mut commands: Commands, query: Query<Entity, With<MenuText>>){
    for e in query.iter(){
        commands.entity(e).despawn();
        println!("Exited menu")
    }
}

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
            .label("menu")
            .with_system(init_menu)
        ).add_system_set(
            SystemSet::on_update(GameState::Menu)
            .label("menu")
            .with_system(start_game)
        ).add_system_set(
            SystemSet::on_exit(GameState::Menu)
            .label("menu")
            .with_system(exit_menu)
        );
    }
}