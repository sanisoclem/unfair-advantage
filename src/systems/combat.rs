use bevy::utils::HashMap;
use crate::systems::{AtlasAnimation, AtlasAnimationDefinition, PhysicsLayers, TimedLife};
use bevy::{math::Vec3Swizzles, prelude::*};
use heron::prelude::*;

#[derive(Debug)]
pub enum CombatEvent {
  DamageApplied(Entity, f32, Vec2, bool),
}

#[derive(Component)]
pub struct Combatant {
  pub hp: f32,
  pub hp_max: f32,
}

#[derive(Component)]
pub struct Immortal;

#[derive(Component, Default)]
pub struct AreaOfEffect {
  pub damage: f32,
  pub tick_timer: Timer,
  pub kill_timer: Timer,
  pub victims: Vec<Entity>,
}

#[derive(Default)]
pub struct FlyingTextSettings {
  pub style: TextStyle,
  pub alignment: TextAlignment,
}


#[derive(Component, Default)]
pub struct Spellbook {
  pub spells: HashMap<SpellType, Spell>,
}

#[derive(Component)]
pub struct ActiveSpell {
  status: ActiveSpellStatus,
  prepare_entity: Option<Entity>
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub enum SpellType {
  BasicAttack
}

pub struct Spell {
  pub status: SpellStatus,
  pub damage: f32,
  pub dot: bool,
  pub damage_tick: f32,
  pub prepare_duration: f32,
  pub cast_duration: f32,
  pub recovery_duration: f32,
  pub prepare_sprite: Option<SpellSprite>,
  pub cast_sprite: Option<SpellSprite>,
  pub recovery_sprite: Option<SpellSprite>,
  pub projectile_sprite: Option<SpellSprite>,
  pub shape: CollisionShape,
  pub projectile_velocity: f32,
}
pub struct SpellSprite {
  pub texture_atlas: Handle<TextureAtlas>,
  pub start_frame: usize,
  pub end_frame: usize,
  pub repeatable: bool,
  pub fps: f32, // only used if repeatable
  pub translation: Vec2 // used to offset the sprite
}
pub enum ActiveSpellStatus {
  NoActiveSpell,
  Preparing(SpellType, Timer),
  Casting(SpellType, Timer),
  Recovery(SpellType, Timer)
}

pub enum SpellStatus {
  Ready,
  Cooldown(Timer)
}


fn find_victims(mut qry: Query<&mut AreaOfEffect>, mut events: EventReader<CollisionEvent>) {
  events
    .iter()
    .filter_map(|event| {
      let (entity_1, entity_2) = event.rigid_body_entities();
      let (layers_1, layers_2) = event.collision_layers();
      if layers_1.contains_group(PhysicsLayers::Attacks)
        && layers_2.contains_group(PhysicsLayers::Enemies)
      {
        Some((entity_2, entity_1, event))
      } else if layers_2.contains_group(PhysicsLayers::Attacks)
        && layers_1.contains_group(PhysicsLayers::Enemies)
      {
        Some((entity_1, entity_2, event))
      } else {
        None
      }
    })
    .for_each(|(enemy, attack, e)| {
      if let Ok(mut aoe) = qry.get_mut(attack) {
        if e.is_started() {
          aoe.victims.push(enemy);
        } else {
          aoe.victims.retain(|victim| *victim != enemy);
        }
      }
    });
}

fn despawn_attacks(
  time: Res<Time>,
  mut to_despawn: Query<(Entity, &mut AreaOfEffect)>,
  mut commands: Commands,
) {
  for (entity, mut aoe) in to_despawn.iter_mut() {
    aoe.kill_timer.tick(time.delta());

    if aoe.kill_timer.just_finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn damage_victims(
  time: Res<Time>,
  mut qry: Query<&mut AreaOfEffect>,
  mut combatant_query: Query<(&mut Combatant, &Transform)>,
  mut events: EventWriter<CombatEvent>,
) {
  for mut aoe in qry.iter_mut() {
    aoe.tick_timer.tick(time.delta());

    if aoe.tick_timer.just_finished() {
      for victim in &aoe.victims {
        if let Ok((mut c, transform)) = combatant_query.get_mut(*victim) {
          c.hp -= aoe.damage;
          if c.hp < 0. {
            c.hp = 0.;
          }
          events.send(CombatEvent::DamageApplied(
            *victim,
            aoe.damage,
            transform.translation.xy(),
            c.hp == 0.,
          ));
        }
      }
    }
  }
}

fn show_damage(
  settings: Res<FlyingTextSettings>,
  mut commands: Commands,
  mut events: EventReader<CombatEvent>,
) {
  for evt in events.iter() {
    match evt {
      CombatEvent::DamageApplied(_victim, damage, pos, _fatal) => {
        commands
          .spawn_bundle(Text2dBundle {
            text: Text::with_section(
              format!("{:?}", *damage as i32),
              settings.style.clone(),
              settings.alignment,
            ),
            transform: Transform::from_translation(Vec3::from((
              pos.clone(),
              crate::z::FLYING_TEXT,
            )))
            .with_scale(Vec3::splat(0.25)),
            ..Default::default()
          })
          .insert(TimedLife::from_seconds(0.1));
        info!("")
      }
    }
  }
}

// fn spawn_aoes(
//   mut commands: Commands,
//   mut actions: EventReader<CombatAction>,
//   asset_server: Res<AssetServer>,
//   mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//   let texture_handle = asset_server.load("combat/Dark VFX 8 (72x32).png");
//   let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(72.0, 32.0), 16, 1);
//   let texture_atlas_handle = texture_atlases.add(texture_atlas);

//   for action in actions.iter() {
//     match action {
//       CombatAction::BasicAttack(origin, direction) => {
//         commands
//           .spawn()
//           .insert(Transform::from_translation(Vec3::new(
//             origin.x,
//             origin.y,
//             crate::z::PLAYER_ATTACK,
//           )))
//           .insert(GlobalTransform::default())
//           .insert(RigidBody::Sensor)
//           .insert(
//             CollisionLayers::none()
//               .with_group(PhysicsLayers::Attacks)
//               .with_mask(PhysicsLayers::Enemies),
//           )
//           .insert(AreaOfEffect {
//             damage: 100.,
//             tick_timer: Timer::from_seconds(0.01, false),
//             kill_timer: Timer::from_seconds(0.8, false),
//             ..Default::default()
//           })
//           .with_children(|parent| {
//             parent
//               .spawn_bundle(SpriteSheetBundle {
//                 texture_atlas: texture_atlas_handle.clone(),
//                 transform: Transform::from_translation(
//                   Vec3::new(direction.x, direction.y, 0.0) * 35.,
//                 )
//                 .with_rotation(Quat::from_rotation_arc(
//                   Vec3::X,
//                   Vec3::new(direction.x, direction.y, 0.0),
//                 )),
//                 ..Default::default()
//               })
//               .insert(AtlasAnimationDefinition {
//                 start: 0,
//                 end: 15,
//                 fps: 20.,
//                 repeat: false,
//                 random_start: false,
//               })
//               .insert(AtlasAnimation::default())
//               .insert(CollisionShape::Cuboid {
//                 half_extends: Vec3::new(25., 10., 0.),
//                 border_radius: None,
//               });
//             parent
//               .spawn()
//               .insert(Transform::from_rotation(Quat::from_rotation_arc(
//                 Vec3::X,
//                 Vec3::new(direction.x, direction.y, 0.0),
//               )))
//               .insert(GlobalTransform::default())
//               .insert(RigidBody::Dynamic)
//               .insert(CollisionShape::Cuboid {
//                 half_extends: Vec3::new(1., 5., 0.),
//                 border_radius: None,
//               })
//               .insert(PhysicMaterial {
//                 density: 100000.0,
//                 ..Default::default()
//               })
//               .insert(Damping::from_linear(5.0))
//               .insert(
//                 CollisionLayers::none()
//                   .with_group(PhysicsLayers::AttackDead)
//                   .with_mask(PhysicsLayers::Corpses),
//               )
//               .insert(Velocity::from_linear(
//                 Vec3::from((direction.clone(), 0.)) * 300.0,
//               ));
//           });
//       }
//       _ => {
//         warn!("unimplemented action {:?}", action);
//       }
//     }
//   }
// }

// pub struct AttackDefinition {
//   pub texture_atlas: Handle<TextureAtlas>,
//   pub idle: AtlasAnimationDefinition,
//   pub attack_frame: usize,
//   pub collission_shape: CollisionShape,
//   pub duration: f32,
// }

fn setup(mut settings: ResMut<FlyingTextSettings>, asset_server: Res<AssetServer>) {
  let font = asset_server.load("FiraMono-Medium.ttf");
  *settings = FlyingTextSettings {
    style: TextStyle {
      font,
      font_size: 30.0,
      color: Color::WHITE,
    },
    alignment: TextAlignment {
      vertical: VerticalAlign::Center,
      horizontal: HorizontalAlign::Center,
    },
  };
}

fn prepare_spells () {}
fn despawn_cancelled_prepared_spells () {} // despawn if has prepare_entity and not preparing
fn cast_spells() {} // update spell status, spawn aoes

#[derive(Component)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<CombatEvent>()
      .init_resource::<FlyingTextSettings>()
      .add_startup_system(setup)
      .add_system(show_damage)
      //.add_system(animate_attack)
      //.add_system(spawn_aoes)
      .add_system(find_victims)
      .add_system(damage_victims)
      .add_system(despawn_attacks);
  }
}
