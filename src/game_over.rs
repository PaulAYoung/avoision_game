use bevy::prelude::*;
use crate::game_structs::Score;
use crate::GameState;
use crate::menu_stuff::{MenuText, get_text_style};


pub fn enter_game_over(
    mut commands: Commands,
    mut score:ResMut<Score>,
    asset_server: Res<AssetServer>
){
    commands
       .spawn_bundle(TextBundle {
            style: get_text_style(),
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                format!("Game Over\nScore:{}\nPress r to restart", score.0.elapsed_secs()),
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


pub fn restart(
    mut game_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::R){
        match game_state.current(){
            GameState::GameOver => {
                game_state.set(GameState::Menu).unwrap();
            }
            _ => {
            }
        }
    }
}