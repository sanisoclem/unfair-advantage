use crate::systems::AtlasAnimation;
use crate::systems::Combatant;
use crate::systems::Immortal;
use crate::systems::SimpleDirection;
use crate::systems::SpellType;
use crate::systems::Spellbook;
use crate::systems::TopDownCharacter;
use bevy::utils::HashMap;
use bevy::{math::Vec3Swizzles, prelude::*};
use heron::prelude::*;
use std::{fmt::Debug, hash::Hash};

use crate::systems::{cleanup_system, CameraTarget, MouseInfo, Movement, PhysicsLayers};

mod animations;
mod spells;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerState {
  Despawned,
  Active,
}

#[derive(Component, Default)]
pub struct PlayerComponent {
  pub dash_speed: f32,
  pub dash_duration: f32,
  pub state: PlayerStateMachine,
}

pub enum PlayerCommand {
  Stop,
  Move(Vec2),
  Dash(Vec2),
  CastSpell(SpellType, Vec2),
}

#[derive(Debug)]
pub enum PlayerStateMachine {
  Idle,
  Running(Vec2),
  Dashing(Vec2, Timer),
  PreparingSpell(SpellType, Vec2, Timer),
  CastingSpell,
  RecoveringFromSpell,
  Bored,
}
impl Default for PlayerStateMachine {
  fn default() -> Self {
    PlayerStateMachine::Idle
  }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerAnimationState {
  Idle,
  Running,
  Dashing,
  PreparingSpell,
  CastingSpell,
  RecoveringFromSpell,
  Bored,
}
impl From<&PlayerStateMachine> for PlayerAnimationState {
  fn from(state: &PlayerStateMachine) -> Self {
    match state {
      PlayerStateMachine::Idle => PlayerAnimationState::Idle,
      PlayerStateMachine::Running(_) => PlayerAnimationState::Running,
      PlayerStateMachine::Dashing(_, _) => PlayerAnimationState::Dashing,
      PlayerStateMachine::PreparingSpell(_, _, _) => PlayerAnimationState::PreparingSpell,
      PlayerStateMachine::CastingSpell => PlayerAnimationState::CastingSpell,
      PlayerStateMachine::RecoveringFromSpell => PlayerAnimationState::RecoveringFromSpell,
      PlayerStateMachine::Bored => PlayerAnimationState::Bored,
    }
  }
}

fn spawn_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("player.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(256.0, 256.0), 24, 17);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  let player = PlayerComponent{
    dash_speed: 1000.,
    dash_duration: 0.5,
    ..Default::default()
  };
  let initial_direction = Vec2::Y * -1.;

  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle.clone(),
      transform: Transform::from_scale(Vec3::splat(0.25)).with_translation(Vec3::new(
        0.0,
        0.0,
        crate::z::PLAYER,
      )),
      ..Default::default()
    })
    // mark this as the camera focus target
    .insert(CameraTarget)
    // this is a top down character
    .insert(AtlasAnimation::default())
    .insert(TopDownCharacter {
      state: PlayerAnimationState::from(&player.state),
      direction_vec: initial_direction,
      direction: SimpleDirection::from(initial_direction),
      animations: animations::build_animations()
    })
    // mark this as the player
    .insert(player)
    // we can get damaged and die
    .insert(Combatant { hp: 1000., hp_max: 1000. })
    // but we are immortal
    .insert(Immortal)
    // we have a spellbook to case spells
    .insert(Spellbook { spells: spells::build_spells(asset_server, texture_atlases) })
    // we can move around
    .insert(Movement {
      speed: 150.0,
      enabled: true,
      target: None,
    })
    // physics
    .insert(RigidBody::KinematicPositionBased)
    .insert(PhysicMaterial {
      friction: 1.0,
      density: 1.,
      ..Default::default()
    })
    .insert(RotationConstraints::lock())
    .insert(
      CollisionLayers::none()
        .with_group(PhysicsLayers::Player)
        .with_mask(PhysicsLayers::Enemies)
        .with_mask(PhysicsLayers::World),
    ).with_children(|builder|{
      builder
        .spawn()
        .insert(Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)))
        .insert(CollisionShape::Sphere {
          radius: 10.
        });
    });
}

fn read_input(
  mouse_button_input: Res<Input<MouseButton>>,
  mouse_info: Res<MouseInfo>,
  keyboard_input: Res<Input<KeyCode>>,
  mut evts: EventWriter<PlayerCommand>,
  mut qry: Query<(&PlayerComponent, &Transform)>,
) {
  if let Ok((_player, transform)) = qry.get_single_mut() {
    let player_pos = transform.translation.xy();
    if mouse_button_input.just_pressed(MouseButton::Left) {
      evts.send(PlayerCommand::Move(mouse_info.world_pos2));
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
      evts.send(PlayerCommand::CastSpell(
        SpellType::BasicAttack,
        (mouse_info.world_pos2 - player_pos).normalize(),
      ));
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
      evts.send(PlayerCommand::Dash((mouse_info.world_pos2 - player_pos).normalize()));
      info!("Sending dash command");
    }
  }
}

fn update_state(
  mut evts: EventReader<PlayerCommand>,
  mut qry: Query<(
    &mut PlayerComponent,
    &mut TopDownCharacter<PlayerAnimationState>,
    &Spellbook,
    &Transform,
  )>,
) {
  if let Ok((mut player, mut character, spellbook, transform)) = qry.get_single_mut() {
    for evt in evts.iter() {
      match (evt, &player.state) {
        (PlayerCommand::Stop, PlayerStateMachine::Running(_))
        | (PlayerCommand::Stop, PlayerStateMachine::PreparingSpell(_,_,_))
        | (PlayerCommand::Stop, PlayerStateMachine::Bored) => {
          player.state = PlayerStateMachine::Idle;
          character.state = PlayerAnimationState::Idle;
        }
        (PlayerCommand::Move(dir), PlayerStateMachine::Running(_))
        | (PlayerCommand::Move(dir), PlayerStateMachine::Idle)
        | (PlayerCommand::Move(dir), PlayerStateMachine::Bored) => {
          player.state = PlayerStateMachine::Running(dir.clone());
          character.state = PlayerAnimationState::Running;
          character.direction_vec = (dir.clone() - transform.translation.xy()).normalize();
        }
        (PlayerCommand::Dash(dir), _) => {
          player.state = PlayerStateMachine::Dashing(dir.clone(), Timer::from_seconds(player.dash_duration, false));
          character.state = PlayerAnimationState::Dashing;
          character.direction_vec = dir.clone();
          info!("set dash state");
        }
        (PlayerCommand::CastSpell(spell_type, dir), PlayerStateMachine::Bored)
        | (PlayerCommand::CastSpell(spell_type, dir), PlayerStateMachine::Running(_))
        | (PlayerCommand::CastSpell(spell_type, dir), PlayerStateMachine::Idle) => {
          if let Some(spell) = spellbook.spells.get(spell_type) {
            player.state = PlayerStateMachine::PreparingSpell(*spell_type, dir.clone(), Timer::from_seconds(spell.prepare_duration, false));
            character.state = PlayerAnimationState::PreparingSpell;
            character.direction_vec = dir.normalize();
          }
        }
        _ => {
          // can't execute command
          warn!("Unknown command");
        }
      }

      info!("player state: {:?}", player.state);
    }
  }
}

fn move_player(mut qry: Query<(&PlayerComponent, &mut Movement), Changed<PlayerComponent>>) {
  for (player, mut mov) in &mut qry.iter_mut() {
    if let PlayerStateMachine::Running(target) = player.state {
      mov.target = Some(target);
    } else {
      mov.target = None;
    }
  }
}

fn stop_when_destination_reached(
  qry: Query<(&PlayerComponent, &Movement), Changed<Movement>>,
  mut evts: EventWriter<PlayerCommand>,
) {
  for (player, mov) in &mut qry.iter() {
    // stop the player if we're running and we've reached the target
    if let (PlayerStateMachine::Running(_), None) = (&player.state, mov.target) {
      evts.send(PlayerCommand::Stop);
    }
  }
}

fn dash_player(time: Res<Time>, mut qry: Query<(&mut PlayerComponent, &mut Transform)>) {
  for (mut player, mut transform) in &mut qry.iter_mut() {
    let speed = player.dash_speed;
    if let PlayerStateMachine::Dashing(dir, timer) = &mut player.state {
      info!("actually dashing");
      timer.tick(time.delta());
      if timer.just_finished() {
        player.state = PlayerStateMachine::Idle;
        info!("stop dashing");
      } else {
        transform.translation.x += dir.x * speed * time.delta_seconds();
        transform.translation.y += dir.y * speed * time.delta_seconds();
      }
    }
  }
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<PlayerCommand>()
      .add_state(PlayerState::Despawned)
      .add_plugin(crate::systems::CharacterPlugin::<PlayerAnimationState>::default())
      .add_system_set(
        SystemSet::on_enter(PlayerState::Despawned).with_system(cleanup_system::<PlayerComponent>),
      )
      .add_system_set(SystemSet::on_exit(PlayerState::Despawned).with_system(spawn_player))
      .add_system_set(
        SystemSet::on_update(PlayerState::Active)
          .with_system(read_input)
          .with_system(move_player)
          .with_system(stop_when_destination_reached)
          .with_system(update_state)
          .with_system(dash_player),

      );
  }
}
