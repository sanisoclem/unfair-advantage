use crate::systems::CombatEvent;
use crate::{
  game::player::PlayerComponent,
  systems::{
    AtlasAnimation, AtlasAnimationDefinition, Combatant, Movement, PhysicsLayers, TimedLife,
  },
};
use bevy::{math::Vec3Swizzles, prelude::*, utils::HashMap};
use heron::prelude::*;
use std::{fmt::Debug, hash::Hash};

pub enum EnemyCommand {
  Spawn(EnemyType, Vec2),
  // SpawnBatch(Vec<(EnemyType, Vec3)>),
  // DespawnAll,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum EnemyType {
  Slime,
  Goblin,
  Boss,
  // Eye,
}

pub struct EnemyDefinition {
  pub texture_atlas: Handle<TextureAtlas>,
  pub idle: AtlasAnimationDefinition,
  pub death: AtlasAnimationDefinition,
  pub max_hp: f32,
  pub fodder: bool,
}

#[derive(Default)]
pub struct EnemyDictionary {
  pub enemies: HashMap<EnemyType, EnemyDefinition>,
}

#[derive(Component)]
pub struct Enemy {
  pub enemy_type: EnemyType,
}

pub fn spawn_enemies(
  mut cmds: EventReader<EnemyCommand>,
  mut commands: Commands,
  enemy_dict: Res<EnemyDictionary>,
) {
  for evt in cmds.iter() {
    match evt {
      EnemyCommand::Spawn(enemy_type, pos) => {
        if let Some(def) = enemy_dict.enemies.get(enemy_type) {
          commands
            .spawn_bundle(SpriteSheetBundle {
              texture_atlas: def.texture_atlas.clone(),
              transform: Transform::from_translation(Vec3::from((pos.clone(), crate::z::ENEMY)))
                .with_scale(Vec3::splat(1.0)),
              ..Default::default()
            })
            .insert(Combatant {
              hp: def.max_hp,
              hp_max: def.max_hp,
            })
            .insert(def.idle.clone())
            .insert(AtlasAnimation::default())
            .insert(Enemy {
              enemy_type: enemy_type.clone(),
            })
            // physics
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 7. })
            //.insert(Velocity::from_linear(Vec3::default()))
            //.insert(Acceleration::from_linear(Vec3::default()))
            .insert(PhysicMaterial {
              friction: 0.2,
              restitution: 1.0,
              density: 1.0,
              ..Default::default()
            })
            .insert(RotationConstraints::lock())
            .insert(Damping::from_linear(3.0))
            .insert(
              CollisionLayers::none()
                .with_group(PhysicsLayers::Enemies)
                .with_mask(PhysicsLayers::Player)
                .with_mask(PhysicsLayers::Attacks)
                .with_mask(PhysicsLayers::Enemies)
                .with_mask(PhysicsLayers::World),
            )
            .insert(Movement {
              speed: 100.0,
              enabled: true,
              target: None,
            })
            .insert(Velocity::from(Vec3::splat(0.0)));
        } else {
          warn!("Enemy type not found: {:?}", enemy_type);
        }
      }
    }
  }
}

fn despawn_dead(
  mut commands: Commands,
  enemy_dic: Res<EnemyDictionary>,
  qry: Query<(&Enemy, &Transform, &Velocity)>,
  qry_killer: Query<&Transform>,
  mut evts: EventReader<CombatEvent>,
) {
  for evt in evts.iter() {
    match evt {
      CombatEvent::CombatantKilled(victim_entity, killer_entity) => {
        commands.entity(*victim_entity).despawn_recursive();
        if let Ok((enemy, transform, v)) = qry.get(*victim_entity) {
          let velocity = if let Ok(killer_transform) = qry_killer.get(*killer_entity) {
            Velocity::from_linear(
              (transform.translation - killer_transform.translation).normalize() * 700.0,
            )
          } else {
            v.clone()
          };

          if let Some(def) = enemy_dic.enemies.get(&enemy.enemy_type) {
            commands
              .spawn_bundle(SpriteSheetBundle {
                texture_atlas: def.texture_atlas.clone(),
                transform: transform.clone(),
                ..Default::default()
              })
              .insert(def.death.clone())
              .insert(RigidBody::Dynamic)
              .insert(CollisionShape::Sphere { radius: 7. })
              .insert(
                CollisionLayers::none()
                  .with_group(PhysicsLayers::Corpses)
                  .with_mask(PhysicsLayers::AttackDead)
                  .with_mask(PhysicsLayers::World),
              )
              .insert(velocity)
              .insert(AtlasAnimation::default())
              .insert(TimedLife::from_seconds(def.death.duration_seconds()));
          }
        }
      }
      _ => {}
    }
  }
}
fn chase_player(
  qry_player: Query<(&PlayerComponent, &Transform), Changed<Transform>>,
  mut qry: Query<(&Enemy, &mut Movement, &Transform)>,
) {
  if let Ok((_, transform)) = qry_player.get_single() {
    for (_, mut mov, t2) in qry.iter_mut() {
      if (t2.translation - transform.translation).length() < 100. {
        mov.target = Some(transform.translation.xy());
      } else {
        mov.target = None;
      }
    }
  }
}

fn setup(
  asset_server: Res<AssetServer>,
  mut enemy_dict: ResMut<EnemyDictionary>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("pack2/full spritesheet2.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 28, 7);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  let mut enemies = HashMap::default();

  enemies.insert(
    EnemyType::Slime,
    EnemyDefinition {
      max_hp: 10.,
      fodder: true,
      texture_atlas: texture_atlas_handle.clone(),
      idle: AtlasAnimationDefinition {
        start: 84,
        end: 89,
        fps: 10.,
        repeat: true,
        random_start: true,
      },
      death: AtlasAnimationDefinition {
        start: 84,
        end: 103,
        fps: 10.,
        repeat: false,
        random_start: false,
      },
    },
  );

  enemies.insert(
    EnemyType::Goblin,
    EnemyDefinition {
      max_hp: 10.,
      fodder: true,
      texture_atlas: texture_atlas_handle.clone(),
      idle: AtlasAnimationDefinition {
        start: 28,
        end: 33,
        fps: 10.,
        repeat: true,
        random_start: true,
      },
      death: AtlasAnimationDefinition {
        start: 28,
        end: 41,
        fps: 10.,
        repeat: false,
        random_start: false,
      },
    },
  );

  let texture_handle = asset_server.load("boss.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(256.0, 256.0), 24, 17);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  enemies.insert(
    EnemyType::Boss,
    EnemyDefinition {
      max_hp: 10.,
      fodder: true,
      texture_atlas: texture_atlas_handle.clone(),
      idle: AtlasAnimationDefinition {
        start: 336,
        end: 336 + 11,
        fps: 10.,
        repeat: true,
        random_start: true,
      },
      death: AtlasAnimationDefinition {
        start: 0,
        end: 23,
        fps: 10.,
        repeat: false,
        random_start: false,
      },
    },
  );

  enemy_dict.enemies = enemies;
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(EnemyDictionary::default())
      .add_event::<EnemyCommand>()
      .add_startup_system(setup)
      .add_system(despawn_dead)
      .add_system(chase_player)
      .add_system(spawn_enemies);
  }
}
