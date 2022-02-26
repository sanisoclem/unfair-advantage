use super::player::PlayerState;
use crate::systems::cleanup_system;
use bevy::prelude::*;
use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelState {
  Disabled,
  TestLevel,
  ProceduralLevel(String),
}

#[derive(Component)]
pub struct LevelTag;

fn setup_test_level(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut player_state: ResMut<State<PlayerState>>,
) {
  player_state
    .set(PlayerState::Active)
    .expect("set player state should always succeed");
}

fn teardown_level(mut player_state: ResMut<State<PlayerState>>) {
  player_state
    .set(PlayerState::Despawned)
    .expect("set player state should always succeed");
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state(LevelState::Disabled)
      .add_system_set(SystemSet::on_enter(LevelState::TestLevel).with_system(setup_test_level))
      .add_system_set(
        SystemSet::on_exit(LevelState::TestLevel)
          .with_system(teardown_level)
          .with_system(cleanup_system::<LevelTag>),
      );
  }
}
