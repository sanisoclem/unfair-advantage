use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use heron::prelude::*;
use std::{fmt::Debug, hash::Hash};

use crate::systems::{
  cleanup_system, CameraTarget, CombatAction, MouseInfo, Movement, PhysicsLayers,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerState {
  Despawned,
  Active,
}

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerComponent;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state(PlayerState::Despawned)
      .add_system_set(
        SystemSet::on_enter(PlayerState::Despawned).with_system(cleanup_system::<PlayerComponent>),
      )
      .add_system_set(SystemSet::on_exit(PlayerState::Despawned).with_system(spawn_player))
      .add_system_set(
        SystemSet::on_update(PlayerState::Active)
          .with_system(read_input)
          .with_system(face_player),
      );
  }
}

fn spawn_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("pack3/spr_character.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle.clone(),
      transform: Transform::from_scale(Vec3::splat(1.0))
        .with_translation(Vec3::new(0.0, 0.0, 100.0)),
      ..Default::default()
    })
    // mark this as the player
    .insert(PlayerComponent)
    // mark this as the camera focus target
    .insert(CameraTarget)
    //physics
    .insert(RigidBody::KinematicPositionBased)
    .insert(CollisionShape::Sphere { radius: 7. })
    //.insert(Velocity::from_linear(Vec3::default()))
    //.insert(Acceleration::from_linear(Vec3::default()))
    .insert(PhysicMaterial {
      friction: 1.0,
      density: 1.0,
      ..Default::default()
    })
    .insert(RotationConstraints::lock())
    .insert(
      CollisionLayers::none()
        .with_group(PhysicsLayers::Player)
        .with_mask(PhysicsLayers::Enemies)
        .with_mask(PhysicsLayers::World),
    )
    // set moveable
    .insert(Movement {
      last_direction: Vec2::Y * -1.,
      speed: 500.0,
      enabled: true,
      target: None,
    });
}
fn face_player(mut qry: Query<(&PlayerComponent, &Movement, &mut TextureAtlasSprite)>) {
  if let Ok((_, mov, mut sprite)) = qry.get_single_mut() {
    let x = mov.last_direction.x;
    let y = mov.last_direction.y;

    if (x.abs() - y.abs()).abs() < 0.35 {
      if x >= 0.0 && y >= 0. {
        // NE
        sprite.index = 3;
      } else if x >= 0. && y < 0. {
        // SE
        sprite.index = 1;
      } else if x < 0. && y >= 0.0 {
        // NW
        sprite.index = 5;
      } else {
        // SW
        sprite.index = 7;
      }
    } else {
      if x.abs() > y.abs() {
        // E
        if x >= 0.0 {
          sprite.index = 2;
        } else {
          // W
          sprite.index = 6;
        }
      } else {
        if y >= 0.0 {
          // N
          sprite.index = 4;
        } else {
          // S
          sprite.index = 0;
        }
      }
    }
  }
}

fn read_input(
  mouse_button_input: Res<Input<MouseButton>>,
  mouse_info: Res<MouseInfo>,
  mut combat_events: EventWriter<CombatAction>,
  mut qry: Query<(&PlayerComponent, &mut Movement, &Transform)>,
) {
  if let Ok((_player, mut mov, transform)) = qry.get_single_mut() {
    if mouse_button_input.just_pressed(MouseButton::Left) {
      mov.target = Some(mouse_info.world_pos2);
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
      let player_pos = transform.translation.xy();
      combat_events.send(CombatAction::BasicAttack(
        player_pos,
        (mouse_info.world_pos2 - player_pos).normalize(),
      ));
    }
  }
}
