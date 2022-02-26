use crate::systems::cleanup_system;
use bevy::prelude::*;
use std::{fmt::Debug, hash::Hash};

use crate::systems::{Animation, MouseInfo, Movement};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerState {
  Despawned,
  Active,
}

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerComponent;

pub enum PlayerAction {
  Move(Vec2),
}

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state(PlayerState::Despawned)
      .add_event::<PlayerAction>()
      .add_system_set(
        SystemSet::on_enter(PlayerState::Despawned).with_system(cleanup_system::<PlayerComponent>),
      )
      .add_system_set(SystemSet::on_exit(PlayerState::Despawned).with_system(spawn_player))
      .add_system_set(
        SystemSet::on_update(PlayerState::Active)
          .with_system(read_input)
          .with_system(execute_player_actions),
      );
  }
}

fn spawn_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("run.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      transform: Transform::from_scale(Vec3::splat(6.0)),
      ..Default::default()
    })
    .insert(PlayerComponent)
    .insert(Movement {
      speed: 500.0,
      enabled: true,
      target: None,
    })
    .insert(Animation::new(10., true));
}

fn read_input(
  mouse_button_input: Res<Input<MouseButton>>,
  mouse_info: Res<MouseInfo>,
  mut player_events: EventWriter<PlayerAction>,
) {
  if mouse_button_input.just_pressed(MouseButton::Left) {
    player_events.send(PlayerAction::Move(mouse_info.world_pos2));
  }
}

fn execute_player_actions(
  mut qry: Query<(&PlayerComponent, &mut Movement)>,
  mut events: EventReader<PlayerAction>,
) {
  for evt in events.iter() {
    match evt {
      PlayerAction::Move(pos) => {
        for (mut _player, mut mov) in qry.iter_mut() {
          mov.target = Some(pos.clone());
        }
      }
    }
  }
}
