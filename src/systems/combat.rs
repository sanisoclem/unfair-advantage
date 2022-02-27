use crate::systems::TimedLife;
use crate::systems::{AtlasAnimation, AtlasAnimationDefinition, PhysicsLayers};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Debug)]
pub enum CombatAction {
  BasicAttack(Vec2, Vec2),
  //WeakAttack,
}

#[derive(Debug)]
pub enum CombatEvent {
  DamageApplied(Entity, f32, Vec2),
}

#[derive(Component)]
pub struct Combatant {
  pub hp: f32,
  pub hp_max: f32,
}

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
          } else {
            events.send(CombatEvent::DamageApplied(
              *victim,
              aoe.damage,
              transform.translation.xy(),
            ));
          }
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
      CombatEvent::DamageApplied(_victim, damage, pos) => {
        commands
          .spawn_bundle(Text2dBundle {
            text: Text::with_section(
              format!("{:?}!", damage),
              settings.style.clone(),
              settings.alignment,
            ),
            transform: Transform::from_translation(Vec3::from((
              pos.clone(),
              crate::z::FLYING_TEXT,
            ))),
            ..Default::default()
          })
          .insert(TimedLife::from_seconds(0.5));
      }
    }
  }
}

fn spawn_aoes(
  mut commands: Commands,
  mut actions: EventReader<CombatAction>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("combat/Dark VFX 8 (72x32).png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(72.0, 32.0), 16, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  for action in actions.iter() {
    match action {
      CombatAction::BasicAttack(origin, direction) => {
        commands
          .spawn()
          .insert(Transform::from_translation(Vec3::new(
            origin.x,
            origin.y,
            crate::z::PLAYER_ATTACK,
          )))
          .insert(GlobalTransform::default())
          //physics
          .insert(RigidBody::KinematicPositionBased)
          .insert(CollisionLayers::none().with_group(PhysicsLayers::Attacks))
          .insert(AreaOfEffect {
            damage: 10.,
            tick_timer: Timer::from_seconds(0.1, true),
            kill_timer: Timer::from_seconds(0.8, false),
            ..Default::default()
          })
          .with_children(|parent| {
            parent
              .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(
                  Vec3::new(direction.x, direction.y, 0.0) * 35.,
                )
                .with_rotation(Quat::from_rotation_arc(
                  Vec3::X,
                  Vec3::new(direction.x, direction.y, 0.0),
                )),
                ..Default::default()
              })
              .insert(AtlasAnimationDefinition {
                start: 0,
                end: 15,
                fps: 20.,
                repeat: false,
                random_start: false,
              })
              .insert(AtlasAnimation::default())
              .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(50., 15., 0.),
                border_radius: None,
              });
          });
      }
      _ => {
        warn!("unimplemented action {:?}", action);
      }
    }
  }
}

// pub struct AttackDefinition {
//   pub texture_atlas: Handle<TextureAtlas>,
//   pub idle: AtlasAnimationDefinition,
//   pub attack_frame: usize,
//   pub collission_shape: CollisionShape,
//   pub duration: f32,
// }

fn setup(mut settings: ResMut<FlyingTextSettings>, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  *settings = FlyingTextSettings {
    style: TextStyle {
      font,
      font_size: 60.0,
      color: Color::WHITE,
    },
    alignment: TextAlignment {
      vertical: VerticalAlign::Center,
      horizontal: HorizontalAlign::Center,
    },
  };
}

#[derive(Component)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<CombatAction>()
      .add_event::<CombatEvent>()
      .init_resource::<FlyingTextSettings>()
      .add_startup_system(setup)
      .add_system(show_damage)
      .add_system(spawn_aoes)
      .add_system(find_victims)
      .add_system(damage_victims)
      .add_system(despawn_attacks);
  }
}
