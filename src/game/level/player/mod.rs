use super::{LevelState, LevelTag};
use crate::game::level::generator::Point;
use crate::systems::{
  AtlasAnimation, CombatAction, Combatant, Immortal, SimpleDirection, SpellType, Spellbook,
  TopDownCharacter,
};
use bevy::{math::Vec3Swizzles, prelude::*};
use heron::{
  prelude::*,
  rapier_plugin::{PhysicsWorld, ShapeCastCollisionType},
};
use std::{fmt::Debug, hash::Hash};

use super::camera::CameraTarget;
use crate::systems::{MouseInfo, Movement, PhysicsLayers};

mod animations;
mod spells;

#[derive(Component, Default)]
pub struct PlayerComponent {
  pub state: PlayerStateMachine,
  pub version: u32, // used to detect if there are actual chanages to state
}

#[derive(Component, Default)]
pub struct ActionQueue {
  pub pending_action: Option<PlayerCommand>, // queue actions to make it more responsive
  pub pending_action_timer: Timer,
}

#[derive(Component, Default)]
pub struct PlayerDash {
  pub origin: Vec2,
  pub direction: Vec2,
  pub distance: f32,
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

#[derive(Clone, Debug)]
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
  // Bored,
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
  // Bored,
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
      // PlayerStateMachine::Bored => PlayerAnimationState::Bored,
    }
  }
}

fn spawn_player(
  level: Res<super::generator::Level>,
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
      transform: Transform::from_scale(Vec3::splat(0.1)).with_translation(Vec3::new(
        level.player_start_position.x as f32 * 16.0, // hax
        level.player_start_position.y as f32 * 16.0,
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
    // unload this entity once level unloads
    .insert(LevelTag)
    // mark this as the player
    .insert(player)
    // we can get damaged and die
    .insert(Combatant { hp: 1., hp_max: 1. })
    // but we are immortal
    .insert(Immortal)
    // queue actions to feel more responsive
    .insert(ActionQueue::default())
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
      ..Default::default()
    })
    // we can dash
    .insert(PlayerDash {
      dash_duration: 0.1,
      cooldown: 0.5,
      ..Default::default()
    })
    // physics
    .insert(RigidBody::KinematicPositionBased)
    .insert(PhysicMaterial {
      friction: 1.0,
      density: 10000.,
      ..Default::default()
    })
    .insert(RotationConstraints::lock())
    .insert(CollisionShape::Sphere { radius: 7. })
    .insert(
      CollisionLayers::none()
        .with_group(PhysicsLayers::Player)
        .with_mask(PhysicsLayers::Enemies)
        .with_mask(PhysicsLayers::World)
        .with_mask(PhysicsLayers::Exit),
    );
}

fn read_input(
  mouse_button_input: Res<Input<MouseButton>>,
  mouse_info: Res<MouseInfo>,
  keyboard_input: Res<Input<KeyCode>>,
  physics_world: PhysicsWorld,
  mut evts: EventWriter<PlayerCommand>,
  mut qry: Query<(&PlayerComponent, &Transform, &CollisionShape)>,
) {
  if let Ok((_player, transform, shape)) = qry.get_single_mut() {
    let player_pos = transform.translation.xy();
    let result = physics_world.shape_cast_with_filter(
      &shape,
      Vec3::from((transform.translation.xy(), 0.)),
      Quat::IDENTITY,
      mouse_info.world_pos3 - Vec3::from((transform.translation.xy(), 0.)),
      CollisionLayers::none()
        .with_group(PhysicsLayers::MovementSensor)
        .with_mask(PhysicsLayers::World),
      |_| true,
    );
    let pos = match result {
      Some(hit) => match hit.collision_type {
        ShapeCastCollisionType::Collided(info) => Some(
          (info.self_end_position.xy() - transform.translation.xy()) * 0.9
            + transform.translation.xy(),
        ),
        _ => None,
      },
      None => Some(mouse_info.world_pos2),
    };

    if mouse_button_input.just_pressed(MouseButton::Left) {
      if let Some(target_pos) = pos {
        evts.send(PlayerCommand::Move(target_pos));
      }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
      evts.send(PlayerCommand::PrepareSpell(
        SpellType::BasicAttack,
        (mouse_info.world_pos2 - player_pos).normalize(),
      ));
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
      if let Some(target_pos) = pos {
        let dist = (target_pos - player_pos).length();
        if dist > 100. {
          // TODO: parameterize max dash distance
          evts.send(PlayerCommand::Dash(
            (target_pos - player_pos).normalize() * 100. + player_pos,
          ));
        } else {
          evts.send(PlayerCommand::Dash(target_pos));
        }
      }
    }
  }
}

fn process_cmd(
  evt: &PlayerCommand,
  character: &mut Mut<TopDownCharacter<PlayerAnimationState>>,
  queue: &mut Mut<ActionQueue>,
  transform: &Transform,
  dash: &PlayerDash,
  player: &mut Mut<PlayerComponent>,
  is_retry: bool,
) -> bool {
  match (evt, &player.state) {
    (PlayerCommand::Stop, _) => {
      player.state = PlayerStateMachine::Idle;
      character.state = PlayerAnimationState::Idle;
      player.version += 1;
      true
    }
    (PlayerCommand::Move(dir), PlayerStateMachine::Running(_))
    | (PlayerCommand::Move(dir), PlayerStateMachine::Idle)
    | (PlayerCommand::Move(dir), PlayerStateMachine::RecoveringFromSpell(_, _))
    | (PlayerCommand::Move(dir), PlayerStateMachine::PreparingSpell(_, _)) => {
      player.state = PlayerStateMachine::Running(dir.clone());
      character.state = PlayerAnimationState::Running;
      character.direction_vec = (dir.clone() - transform.translation.xy()).normalize();
      player.version += 1;
      true
    }
    (PlayerCommand::Dash(dir), _) => {
      if !dash.on_cooldown {
        info!("dash! {:?}", dir);
        player.state = PlayerStateMachine::Dashing(dir.clone());
        character.state = PlayerAnimationState::Dashing;
        character.direction_vec = (dir.clone() - transform.translation.xy()).normalize();
        player.version += 1;
      }
      true
    }
    (PlayerCommand::PrepareSpell(spell_type, dir), PlayerStateMachine::Running(_))
    | (PlayerCommand::PrepareSpell(spell_type, dir), PlayerStateMachine::Idle) => {
      player.state = PlayerStateMachine::PreparingSpell(*spell_type, dir.clone());
      character.state = PlayerAnimationState::PreparingSpell;
      character.direction_vec = dir.normalize();
      player.version += 1;
      true
    }
    (PlayerCommand::CastSpell, PlayerStateMachine::PreparingSpell(spell_type, dir)) => {
      let st = spell_type.clone();
      let d = dir.clone();
      player.state = PlayerStateMachine::CastingSpell(st, d);
      character.state = PlayerAnimationState::CastingSpell;
      character.direction_vec = d.normalize();
      player.version += 1;
      true
    }
    (PlayerCommand::RecoverFromSpell, PlayerStateMachine::CastingSpell(spell_type, dir)) => {
      let st = spell_type.clone();
      let d = dir.clone();
      player.state = PlayerStateMachine::RecoveringFromSpell(st, d);
      character.state = PlayerAnimationState::RecoveringFromSpell;
      character.direction_vec = d.normalize();
      player.version += 1;
      true
    }
    (cmd, _) => {
      // can't execute command
      warn!("Unknown command");
      if !is_retry {
        queue.pending_action = Some(cmd.clone());
        queue.pending_action_timer = Timer::from_seconds(0.2, false);
      }
      false
    }
  }
}

fn update_state(
  mut evts: EventReader<PlayerCommand>,
  mut qry: Query<(
    &mut PlayerComponent,
    &mut ActionQueue,
    &mut TopDownCharacter<PlayerAnimationState>,
    &PlayerDash,
    &Transform,
  )>,
  time: Res<Time>,
) {
  if let Ok((mut player, mut queue, mut character, dash, transform)) = qry.get_single_mut() {
    let mut event_processed = false;
    for evt in evts.iter() {
      event_processed =
        event_processed || process_cmd(evt, &mut character, &mut queue, transform, dash, &mut player, false);
    }

    if let Some(cmd) = (&queue.pending_action).as_ref().map(|cmd| cmd.clone()) {
      // pending action, try and execute it
      if process_cmd(&cmd, &mut character, &mut queue, transform, dash, &mut player, true) {
        queue.pending_action = None;
        queue.pending_action_timer.reset();
      } else {
        queue.pending_action_timer.tick(time.delta());
        if queue.pending_action_timer.just_finished() {
          queue.pending_action = None;
          queue.pending_action_timer.reset();
        }
      }
    }
  }
}

fn sync_spells(
  mut qry: Query<
    (Entity, &PlayerComponent, &mut PlayerSpells, &Spellbook),
    Changed<PlayerComponent>,
  >,
  mut evts: EventWriter<CombatAction>,
) {
  for (entity, player, mut active_spell, spells) in &mut qry.iter_mut() {
    if active_spell.player_state_version == player.version {
      continue;
    }

    match &player.state {
      PlayerStateMachine::PreparingSpell(spell_type, dir) => {
        let spell = spells.spells.get(spell_type).expect("should find spell");
        active_spell.timer = Timer::from_seconds(spell.prepare_duration, false);
        evts.send(CombatAction::PrepareSpell(
          entity,
          spell_type.clone(),
          dir.clone(),
        ));
      }
      PlayerStateMachine::CastingSpell(spell_type, dir) => {
        let spell = spells.spells.get(spell_type).expect("should find spell");
        active_spell.timer = Timer::from_seconds(spell.cast_duration, false);
        evts.send(CombatAction::CastSpell(
          entity,
          spell_type.clone(),
          dir.clone(),
        ));
      }
      PlayerStateMachine::RecoveringFromSpell(spell_type, dir) => {
        let spell = spells.spells.get(spell_type).expect("should find spell");
        active_spell.timer = Timer::from_seconds(spell.recovery_duration, false);
        evts.send(CombatAction::RecoverFromSpell(
          entity,
          spell_type.clone(),
          dir.clone(),
        ));
      }
      _ => {
        if !active_spell.timer.finished() {
          active_spell.timer.reset();
          evts.send(CombatAction::CancelSpell(entity));
        }
      }
    }
  }
}

fn run_spells(
  time: Res<Time>,
  mut qry: Query<(&PlayerComponent, &mut PlayerSpells)>,
  mut evts: EventWriter<PlayerCommand>,
) {
  for (player, mut active_spell) in &mut qry.iter_mut() {
    active_spell.timer.tick(time.delta());

    if active_spell.timer.just_finished() {
      match &player.state {
        PlayerStateMachine::PreparingSpell(_, _) => {
          evts.send(PlayerCommand::CastSpell);
        }
        PlayerStateMachine::CastingSpell(_, _) => {
          evts.send(PlayerCommand::RecoverFromSpell);
        }
        PlayerStateMachine::RecoveringFromSpell(_, _) => {
          evts.send(PlayerCommand::Stop);
        }
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
      mov.path_backlog.clear();
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
  mut qry: Query<(&PlayerComponent, &mut PlayerDash, &mut Transform), Changed<PlayerComponent>>,
) {
  for (player, mut dash, mut transform) in &mut qry.iter_mut() {
    if let PlayerStateMachine::Dashing(target) = &player.state {
      if player.version > dash.player_state_version {
        dash.origin = transform.translation.xy();
        dash.direction = (target.clone() - dash.origin).normalize();
        dash.distance = (target.clone() - dash.origin).length();
        dash.timer = Timer::from_seconds(dash.dash_duration, false);
        dash.cd_timer = Timer::from_seconds(dash.cooldown, false);
        dash.on_cooldown = true;
        dash.player_state_version = player.version;

        transform.scale.x = 0.03;
        transform.scale.y = 0.03;
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
    if let PlayerStateMachine::Dashing(_) = &player.state {
      let increments = (dash.direction * dash.distance) / dash.dash_duration;

      transform.translation.x += increments.x * time.delta_seconds();
      transform.translation.y += increments.y * time.delta_seconds();
      dash.timer.tick(time.delta());
      if dash.timer.just_finished() {
        evts.send(PlayerCommand::Stop);
        transform.scale.x = 0.1;
        transform.scale.y = 0.1;
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
      .add_plugin(crate::systems::CharacterPlugin::<PlayerAnimationState>::default())
      .add_system_set(SystemSet::on_enter(LevelState::Loaded).with_system(spawn_player))
      .add_system_set(
        SystemSet::on_update(LevelState::Loaded)
          .with_system(read_input.label("input"))
          .with_system(update_state.label("update_state").after("input"))
          .with_system(move_player.label("update_move").after("update_state"))
          .with_system(
            stop_when_destination_reached
              .label("stop_move")
              .after("update_move"),
          )
          .with_system(start_dash_player.label("start_dash").after("update_state"))
          .with_system(dash_player.label("update_dash").after("start_dash"))
          .with_system(sync_spells)
          .with_system(run_spells),
      );
  }
}
