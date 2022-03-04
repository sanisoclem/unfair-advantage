use super::{
  generator::{Level, Point},
  player::PlayerComponent,
};
use super::{LevelState, LevelTag};
use crate::systems::Combatant;
use crate::systems::Movement;
use crate::systems::{AtlasAnimation, CombatEvent, PhysicsLayers, TimedLife};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use heron::prelude::*;
use rand::prelude::*;

mod definitions;
pub use definitions::*;

#[derive(Component)]
pub struct Enemy {
  pub enemy_type: EnemyType,
  pub spawn_point: Point,
}

pub struct SpawnTimer {
  pub timer: Timer,
}
impl Default for SpawnTimer {
  fn default() -> Self {
    SpawnTimer {
      timer: Timer::from_seconds(1.0, true),
    }
  }
}

pub fn spawn_enemies(
  time: Res<Time>,
  mut commands: Commands,
  mut level: ResMut<Level>,
  mut timer: ResMut<SpawnTimer>,
  enemy_dict: Res<EnemyDictionary>,
  qry: Query<&Transform, With<PlayerComponent>>,
) {
  timer.timer.tick(time.delta());
  if !timer.timer.just_finished() { return }

  if let Ok(player_transform) = qry.get_single() {
    let mut rng = rand::thread_rng();
    for (x, y, mut tile) in level.get_tiles_mut() {
      let pos = Vec2::new(x as f32, y as f32) * 16.0;
      if !tile.is_spawn_point
        || tile.spawned
        || (pos - player_transform.translation.xy()).length().abs() > 300.
      {
        continue;
      }

      let enemy_type = match rng.gen_range(0..10u8) {
        0 | 1 | 2 | 3 | 4 | 5  => EnemyType::Slime,
        6 | 7 => EnemyType::Eye,
        _ => EnemyType::Goblin,
      };
      let def = enemy_dict
        .enemies
        .get(&enemy_type)
        .expect("Enemy type not found");

      tile.spawned = true;

      commands
        .spawn_bundle(SpriteSheetBundle {
          texture_atlas: def.texture_atlas.clone(),
          transform: Transform::from_translation(Vec3::from((pos, crate::z::ENEMY))),
          ..Default::default()
        })
        .insert(Combatant {
          hp: def.max_hp,
          hp_max: def.max_hp,
        })
        .insert(LevelTag)
        .insert(def.idle.clone())
        .insert(AtlasAnimation::default())
        .insert(Enemy {
          enemy_type: enemy_type.clone(),
          spawn_point: Point { x, y },
        })
        // physics
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 7. })
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
          ..Default::default()
        })
        .insert(Velocity::from(Vec3::splat(0.0)));
    }
  }
}

fn despawn_dead(
  mut commands: Commands,
  mut level: ResMut<Level>,
  enemy_dic: Res<EnemyDictionary>,
  qry: Query<(&Enemy, &Transform, &Velocity)>,
  qry_killer: Query<&Transform>,
  mut evts: EventReader<CombatEvent>,
) {
  for evt in evts.iter() {
    match evt {
      CombatEvent::CombatantKilled(victim_entity, killer_entity) => {
        let (enemy, transform, v) = qry.get(*victim_entity).expect("Enemy not found");
        let mut spawner = level.get_tile_mut(enemy.spawn_point.x, enemy.spawn_point.y);
        let def = enemy_dic
          .enemies
          .get(&enemy.enemy_type)
          .expect("Enemy type not found");
        let velocity = if let Ok(killer_transform) = qry_killer.get(*killer_entity) {
          Velocity::from_linear(
            (transform.translation - killer_transform.translation).normalize() * 700.0,
          )
        } else {
          v.clone()
        };

        commands
          .spawn_bundle(SpriteSheetBundle {
            texture_atlas: def.texture_atlas.clone(),
            transform: transform.clone(),
            ..Default::default()
          })
          .insert(LevelTag)
          .insert(def.death.clone())
          .insert(RigidBody::Dynamic)
          .insert(PhysicMaterial {
            restitution: 0.4,
            ..Default::default()
          })
          .insert(CollisionShape::Sphere { radius: 7. })
          .insert(
            CollisionLayers::none()
              .with_group(PhysicsLayers::Corpses)
              .with_mask(PhysicsLayers::World),
          )
          .insert(velocity)
          .insert(AtlasAnimation::default())
          .insert(TimedLife::from_seconds(def.death.duration_seconds()));

        commands.entity(*victim_entity).despawn_recursive();
        spawner.spawned = false;
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
      if (t2.translation - transform.translation).length() < 300. {
        mov.target = Some(transform.translation.xy());
      } else {
        mov.target = None;
      }
    }
  }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<EnemyDictionary>()
      .init_resource::<SpawnTimer>()
      .add_system_set(
      SystemSet::on_update(LevelState::Loaded)
        .with_system(despawn_dead)
        .with_system(chase_player)
        .with_system(spawn_enemies),
    );
  }
}
