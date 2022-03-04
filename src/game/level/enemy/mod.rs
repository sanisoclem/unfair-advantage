use super::{LevelState, LevelTag};
use crate::systems::{AtlasAnimation, CombatEvent, PhysicsLayers, TimedLife};
use bevy::prelude::*;
use heron::prelude::*;

mod definitions;
pub use definitions::*;

#[derive(Component)]
pub struct Enemy {
  pub enemy_type: EnemyType,
}

pub fn spawn_enemies(mut commands: Commands, enemy_dict: Res<EnemyDictionary>) {
  // for evt in cmds.iter() {
  //   match evt {
  //     EnemyCommand::Spawn(enemy_type, pos) => {
  //       if let Some(def) = enemy_dict.enemies.get(enemy_type) {
  //         commands
  //           .spawn_bundle(SpriteSheetBundle {
  //             texture_atlas: def.texture_atlas.clone(),
  //             transform: Transform::from_translation(Vec3::from((pos.clone(), crate::z::ENEMY)))
  //               .with_scale(Vec3::splat(1.0)),
  //             ..Default::default()
  //           })
  //           .insert(Combatant {
  //             hp: def.max_hp,
  //             hp_max: def.max_hp,
  //           })
  //           .insert(LevelTag)
  //           .insert(def.idle.clone())
  //           .insert(AtlasAnimation::default())
  //           .insert(Enemy {
  //             enemy_type: enemy_type.clone(),
  //           })
  //           // physics
  //           .insert(RigidBody::Dynamic)
  //           .insert(CollisionShape::Sphere { radius: 7. })
  //           //.insert(Velocity::from_linear(Vec3::default()))
  //           //.insert(Acceleration::from_linear(Vec3::default()))
  //           .insert(PhysicMaterial {
  //             friction: 0.2,
  //             restitution: 1.0,
  //             density: 1.0,
  //             ..Default::default()
  //           })
  //           .insert(RotationConstraints::lock())
  //           .insert(Damping::from_linear(3.0))
  //           .insert(
  //             CollisionLayers::none()
  //               .with_group(PhysicsLayers::Enemies)
  //               .with_mask(PhysicsLayers::Player)
  //               .with_mask(PhysicsLayers::Attacks)
  //               .with_mask(PhysicsLayers::Enemies)
  //               .with_mask(PhysicsLayers::World),
  //           )
  //           .insert(Movement {
  //             speed: 100.0,
  //             enabled: true,
  //             target: None,
  //           })
  //           .insert(Velocity::from(Vec3::splat(0.0)));
  //       } else {
  //         warn!("Enemy type not found: {:?}", enemy_type);
  //       }
  //     }
  //   }
  // }
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
          }
        }
      }
      _ => {}
    }
  }
}

// fn chase_player(
//   qry_player: Query<(&PlayerComponent, &Transform), Changed<Transform>>,
//   mut qry: Query<(&Enemy, &mut Movement, &Transform)>,
// ) {
//   if let Ok((_, transform)) = qry_player.get_single() {
//     for (_, mut mov, t2) in qry.iter_mut() {
//       if (t2.translation - transform.translation).length() < 100. {
//         mov.target = Some(transform.translation.xy());
//       } else {
//         mov.target = None;
//       }
//     }
//   }
// }

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<EnemyDictionary>().add_system_set(
      SystemSet::on_update(LevelState::Loaded)
        .with_system(despawn_dead)
        //.with_system(chase_player)
        .with_system(spawn_enemies),
    );
  }
}
