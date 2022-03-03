use crate::systems::CombatAction;
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

#[derive(Component, Default)]
pub struct PlayerComponent {
  pub state: PlayerStateMachine,
  pub version: u32, // used to detect if there are actual chanages to state
}

#[derive(Component, Default)]
pub struct PlayerDash {
  pub dash_speed: f32,
  pub dash_duration: f32,
  pub cooldown: f32,
  pub on_cooldown: bool,
  pub timer: Timer,
  pub cd_timer: Timer,
  pub player_state_version: u32,
}

#[derive(Component, Default)]
pub struct PlayerSpells {
  pub timer: Timer,
  pub player_state_version: u32,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerState {
  Despawned,
  Active,
}

pub enum PlayerCommand {
  Stop,
  Move(Vec2),
  Dash(Vec2),
  PrepareSpell(SpellType, Vec2),
  CastSpell,
  RecoverFromSpell,
}

#[derive(Debug)]
pub enum PlayerStateMachine {
  Idle,
  Running(Vec2),
  Dashing(Vec2),
  PreparingSpell(SpellType, Vec2),
  CastingSpell(SpellType, Vec2),
  RecoveringFromSpell(SpellType, Vec2),
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
      PlayerStateMachine::Dashing(_) => PlayerAnimationState::Dashing,
      PlayerStateMachine::PreparingSpell(_, _) => PlayerAnimationState::PreparingSpell,
      PlayerStateMachine::CastingSpell(_, _) => PlayerAnimationState::CastingSpell,
      PlayerStateMachine::RecoveringFromSpell(_, _) => PlayerAnimationState::RecoveringFromSpell,
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

  let player = PlayerComponent::default();
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
      animations: animations::build_animations(),
    })
    // mark this as the player
    .insert(player)
    // we can get damaged and die
    .insert(Combatant {
      hp: 1.,
      hp_max: 1.,
    })
    // but we are immortal
    .insert(Immortal)
    // we have a spellbook to cast spells
    .insert(Spellbook {
      spells: spells::build_spells(asset_server, texture_atlases),
    })
    // we can cast spells
    .insert(PlayerSpells::default())
    // we can move around
    .insert(Movement {
      speed: 150.0,
      enabled: true,
      target: None,
    })
    // we can dash
    .insert(PlayerDash {
      dash_speed: 500.0,
      dash_duration: 0.3,
      cooldown: 2.0,
      ..Default::default()
    })
    // physics
    .insert(RigidBody::Dynamic)
    .insert(PhysicMaterial {
      friction: 1.0,
      density: 10000.,
      ..Default::default()
    })
    .insert(RotationConstraints::lock())
    .insert(CollisionShape::Sphere { radius: 10. })
    .insert(
      CollisionLayers::none()
        .with_group(PhysicsLayers::Player)
        .with_mask(PhysicsLayers::Enemies)
        .with_mask(PhysicsLayers::World),
    )
    .with_children(|builder| {
      builder
        .spawn()
        .insert(Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)))
        .insert(CollisionShape::Sphere { radius: 10. });
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
      evts.send(PlayerCommand::PrepareSpell(
        SpellType::BasicAttack,
        (mouse_info.world_pos2 - player_pos).normalize(),
      ));
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
      evts.send(PlayerCommand::Dash(
        (mouse_info.world_pos2 - player_pos).normalize(),
      ));
      //info!("Sending dash command");
    }
  }
}

fn update_state(
  mut evts: EventReader<PlayerCommand>,
  mut qry: Query<(
    &mut PlayerComponent,
    &mut TopDownCharacter<PlayerAnimationState>,
    &PlayerDash,
    &Transform,
  )>,
) {
  if let Ok((mut player, mut character, dash, transform)) = qry.get_single_mut() {
    for evt in evts.iter() {
      match (evt, &player.state) {
        (PlayerCommand::Stop, _) => {
          player.state = PlayerStateMachine::Idle;
          character.state = PlayerAnimationState::Idle;
          player.version += 1;
        }
        (PlayerCommand::Move(dir), PlayerStateMachine::Running(_))
        | (PlayerCommand::Move(dir), PlayerStateMachine::Idle)
        | (PlayerCommand::Move(dir), PlayerStateMachine::RecoveringFromSpell(_,_))
        | (PlayerCommand::Move(dir), PlayerStateMachine::PreparingSpell(_,_))
        | (PlayerCommand::Move(dir), PlayerStateMachine::Bored) => {
          player.state = PlayerStateMachine::Running(dir.clone());
          character.state = PlayerAnimationState::Running;
          character.direction_vec = (dir.clone() - transform.translation.xy()).normalize();
          player.version += 1;
        }
        (PlayerCommand::Dash(dir), _) => {
          if !dash.on_cooldown {
            player.state = PlayerStateMachine::Dashing(dir.clone());
            character.state = PlayerAnimationState::Dashing;
            character.direction_vec = dir.clone();
            player.version += 1;
            //info!("set dash state");
          }
        }
        (PlayerCommand::PrepareSpell(spell_type, dir), PlayerStateMachine::Bored)
        | (PlayerCommand::PrepareSpell(spell_type, dir), PlayerStateMachine::Running(_))
        | (PlayerCommand::PrepareSpell(spell_type, dir), PlayerStateMachine::Idle) => {
          player.state = PlayerStateMachine::PreparingSpell(*spell_type, dir.clone());
          character.state = PlayerAnimationState::PreparingSpell;
          character.direction_vec = dir.normalize();
          player.version += 1;
        },
        (PlayerCommand::CastSpell, PlayerStateMachine::PreparingSpell(spell_type, dir)) => {
          let st = spell_type.clone();
          let d = dir.clone();
          player.state = PlayerStateMachine::CastingSpell(st, d);
          character.state = PlayerAnimationState::CastingSpell;
          character.direction_vec = d.normalize();
          player.version += 1;
        },
        (PlayerCommand::RecoverFromSpell, PlayerStateMachine::CastingSpell(spell_type, dir)) => {
          let st = spell_type.clone();
          let d = dir.clone();
          player.state = PlayerStateMachine::RecoveringFromSpell(st, d);
          character.state = PlayerAnimationState::RecoveringFromSpell;
          character.direction_vec = d.normalize();
          player.version += 1;
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

fn sync_spells(
  mut qry: Query<(Entity, &PlayerComponent, &mut PlayerSpells, &Spellbook), Changed<PlayerComponent>>,
  mut evts: EventWriter<CombatAction>
) {
  for (entity, player, mut active_spell, spells) in &mut qry.iter_mut() {
    if active_spell.player_state_version == player.version {
      continue;
    }

    match &player.state {
      PlayerStateMachine::PreparingSpell(spell_type, dir) => {
        let spell = spells.spells.get(spell_type).expect("should find spell");
        active_spell.timer = Timer::from_seconds(spell.prepare_duration, false);
        evts.send(CombatAction::PrepareSpell(entity, spell_type.clone(), dir.clone()));
      },
      PlayerStateMachine::CastingSpell(spell_type, dir) => {
        let spell = spells.spells.get(spell_type).expect("should find spell");
        active_spell.timer = Timer::from_seconds(spell.cast_duration, false);
        evts.send(CombatAction::CastSpell(entity, spell_type.clone(), dir.clone()));
      },
      PlayerStateMachine::RecoveringFromSpell(spell_type, dir) => {
        let spell = spells.spells.get(spell_type).expect("should find spell");
        active_spell.timer = Timer::from_seconds(spell.recovery_duration, false);
        evts.send(CombatAction::RecoverFromSpell(entity, spell_type.clone(), dir.clone()));
      },
      _ => {
        if !active_spell.timer.finished() {
          active_spell.timer.reset();
          evts.send(CombatAction::CancelSpell(entity));
        }
      }
    }
  }
}

fn run_spells(time: Res<Time>, mut qry: Query<(&PlayerComponent, &mut PlayerSpells)>, mut evts: EventWriter<PlayerCommand>) {
  for (player, mut active_spell) in &mut qry.iter_mut() {

    active_spell.timer.tick(time.delta());

    if active_spell.timer.just_finished() {
      match &player.state {
        PlayerStateMachine::PreparingSpell(_, _) => {
          evts.send(PlayerCommand::CastSpell);
        },
        PlayerStateMachine::CastingSpell(_, _) => {
          evts.send(PlayerCommand::RecoverFromSpell);
        },
        PlayerStateMachine::RecoveringFromSpell(_, _) => {
          evts.send(PlayerCommand::Stop);
        },
        _ => {}
      }
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

fn start_dash_player(
  mut qry: Query<(&PlayerComponent, &mut PlayerDash), Changed<PlayerComponent>>,
) {
  for (player, mut dash) in &mut qry.iter_mut() {
    if let PlayerStateMachine::Dashing(_) = &player.state {
      if player.version > dash.player_state_version {
        //info!("Start dashing");
        dash.player_state_version = player.version;
        dash.timer = Timer::from_seconds(dash.dash_duration, false);
        dash.cd_timer = Timer::from_seconds(dash.cooldown, false);
        dash.on_cooldown = true;
      }
    }
  }
}
fn dash_player(
  time: Res<Time>,
  mut qry: Query<(&PlayerComponent, &mut Transform, &mut PlayerDash)>,
  mut evts: EventWriter<PlayerCommand>,
) {
  for (player, mut transform, mut dash) in &mut qry.iter_mut() {
    let speed = dash.dash_speed;

    if let PlayerStateMachine::Dashing(dir) = &player.state {
      dash.timer.tick(time.delta());
      if dash.timer.just_finished() {
        evts.send(PlayerCommand::Stop);
        //info!("stop dashing");
      } else {
        transform.translation.x += dir.x * speed * time.delta_seconds();
        transform.translation.y += dir.y * speed * time.delta_seconds();
      }
    } else {
      dash.cd_timer.tick(time.delta());
      if dash.cd_timer.just_finished() {
        dash.on_cooldown = false;
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
          .with_system(start_dash_player)
          .with_system(dash_player)
          .with_system(sync_spells)
          .with_system(run_spells),
      );
  }
}
