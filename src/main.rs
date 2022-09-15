use avoidee::{AvoideeSpawnEvent, spawn_avoidee_event_reader};
use avoider::{AvoiderSpawnEvent, spawn_avoider_event_reader};
use bevy::{prelude::*, core::Stopwatch};

mod materials;
mod avoider;
mod avoidee;
mod constants;
mod systems;
mod game_structs;
mod gep;
mod menu_stuff;
mod game_over;

use gep::{Position};
use constants::{POSITION_SCALE, ARENA_HEIGHT, ARENA_WIDTH};
use game_structs::{GameState, Score};
use menu_stuff::{init_score, update_score};


fn position_scale(mut q: Query<(&Position, &mut Transform)>){
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = (pos.0*POSITION_SCALE).extend(0.0);
    }
}

fn setup(mut commands: Commands){
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
fn main(){
    App::new()
    .insert_resource(WindowDescriptor{
        title: "Avoision".to_string(),
        width: ARENA_WIDTH as f32 *POSITION_SCALE,
        height: ARENA_HEIGHT as f32 *POSITION_SCALE,
        ..Default::default()
    })
    .add_state(GameState::Menu)
    .add_event::<AvoiderSpawnEvent>()
    .add_event::<AvoideeSpawnEvent>()
    .add_startup_system(materials::setup_materials)
    .add_startup_system(setup)
    .insert_resource(Score(Stopwatch::new()))
    .add_system_set(
        SystemSet::on_enter(GameState::InGame)
        .with_system(systems::setup_game)
        .with_system(init_score)
    )
    .add_system_set(
        SystemSet::on_exit(GameState::InGame)
        .with_system(systems::tear_down)
    )
    .add_system_set(
        SystemSet::on_update(GameState::InGame)
        .with_system(spawn_avoider_event_reader)
        .with_system(spawn_avoidee_event_reader)
        .with_system(position_scale)
        .with_system(avoider::avoider_movement)
        .with_system(systems::loop_space)
        .with_system(update_score)
        .with_system(systems::detect_player_collision)
        .with_system(systems::avoidee_spawner.before(position_scale))
    )
    .add_system_set(
        SystemSet::on_update(GameState::Paused)
    )
    .add_system_set(
        SystemSet::on_enter(GameState::GameOver)
        .with_system(game_over::enter_game_over)
    )
    .add_system_set(
        SystemSet::on_exit(GameState::GameOver)
        .with_system(menu_stuff::exit_menu)
        .with_system(systems::tear_down)
    )
    .add_system_set(SystemSet::on_update(GameState::GameOver)
        .with_system(game_over::restart)
    )
    .add_system(systems::pause_unpause)
    .add_plugins(DefaultPlugins)
    .add_plugin(gep::plugin::GEPPlugin)
    .add_plugin(menu_stuff::MenuPlugin)
    .run();
}