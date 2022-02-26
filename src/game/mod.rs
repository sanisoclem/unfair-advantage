use super::GameState;
use bevy::prelude::*;

pub mod level;
pub mod player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(level::LevelPlugin)
      .add_plugin(player::PlayerPlugin)
      .add_system_set(SystemSet::on_enter(GameState::Game).with_system(load_test_level))
      .add_system_set(SystemSet::on_exit(GameState::Game).with_system(unload_level));
  }
}

fn load_test_level(mut level_state: ResMut<State<level::LevelState>>) {
  level_state
    .set(level::LevelState::TestLevel)
    .expect("set level state should always succeed");
}

fn unload_level(mut level_state: ResMut<State<level::LevelState>>) {
  level_state
    .set(level::LevelState::Disabled)
    .expect("set level state should always succeed");
}
