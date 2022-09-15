use bevy::prelude::*;
use crate::game_structs::Score;
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
                format!("Game Over\nScore:{}", score.0.elapsed_secs()),
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