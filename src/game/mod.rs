use bevy::prelude::*;

use super::{despawn_screen, GameState};

mod player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(player::PlayerPlugin::create(OnGameScreen, GameState::Game, GameState::Menu))
      .add_system_set(
        SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
      );
  }
}

// Tag component used to tag entities added on the game screen
#[derive(Component, Default)]
struct OnGameScreen;

