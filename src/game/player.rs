use bevy::prelude::*;
use std::{fmt::Debug, hash::Hash};

use super::{mouse::MouseInfo, movement::Movement};
use crate::animation::Animation;

pub struct PlayerPlugin<T, TState> {
  pub tag: T,
  pub game_state: TState,
  pub end_state: TState,
}

#[derive(Component)]
pub struct PlayerComponent;

pub enum PlayerAction {
  Move(Vec2),
}

impl<T, TState> Plugin for PlayerPlugin<T, TState>
where
  T: Component + Default,
  TState: Hash + Debug + Eq + Clone + Copy + Sync + Send + 'static,
{
  fn build(&self, app: &mut App) {
    app
      .add_event::<PlayerAction>()
      .add_system_set(SystemSet::on_enter(self.game_state).with_system(Self::setup_player))
      .add_system_set(
        SystemSet::on_update(self.game_state)
          .with_system(Self::read_input)
          .with_system(Self::execute_player_actions),
      );
  }
}

impl<T, TState> PlayerPlugin<T, TState>
where
  T: Component + Default,
{
  pub fn create(tag: T, game_state: TState, end_state: TState) -> Self {
    Self {
      tag,
      game_state,
      end_state,
    }
  }
  fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  ) {
    let texture_handle = asset_server.load("run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
      .spawn_bundle(OrthographicCameraBundle::new_2d())
      .insert(T::default());

    commands
      .spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..Default::default()
      })
      .insert(T::default())
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

    if mouse_button_input.just_pressed(MouseButton::Left) {
      info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
      info!("left mouse just released");
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
        },
      }
    }
  }
}
