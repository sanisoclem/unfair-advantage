use crate::systems::{AtlasAnimation, AtlasAnimationDefinition, PhysicsLayers, TimedLife};
use rand::distributions::{Distribution, Uniform};
use bevy::utils::HashMap;
use bevy::{math::Vec3Swizzles, prelude::*};
use heron::prelude::*;

#[derive(Debug)]
pub enum CombatEvent {
  DamageApplied(Entity, f32, Vec2, bool),
  CombatantKilled(Entity, Entity),
}

#[derive(Debug)]
pub enum CombatAction {
  PrepareSpell(Entity, SpellType, Vec2),
  CastSpell(Entity, SpellType, Vec2),
  RecoverFromSpell(Entity, SpellType, Vec2),
  CancelSpell(Entity),
}

#[derive(Component)]
pub struct Combatant {
  pub hp: f32,
  pub hp_max: f32,
}

#[derive(Component)]
pub struct Immortal;

#[derive(Component)]
pub struct AreaOfEffect {
  pub caster: Entity,
  pub damage_min: f32,
  pub damage_max: f32,
  pub tick_timer: Timer,
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

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub enum SpellType {
  BasicAttack,
}

pub struct Spell {
  pub status: SpellStatus,
  pub damage_min: f32,
  pub damage_max: f32,
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
  pub fps: f32,          // only used if repeatable
  pub translation: Vec2, // used to offset the sprite
}

pub enum SpellStatus {
  Ready,
  Cooldown(Timer),
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
          info!("victim added");
          aoe.victims.push(enemy);
        } else {
          info!("victim removed");
          aoe.victims.retain(|victim| *victim != enemy);
        }
      }
    });
}

fn damage_victims(
  time: Res<Time>,
  mut qry: Query<&mut AreaOfEffect>,
  mut combatant_query: Query<(Entity, &mut Combatant, &Transform)>,
  mut events: EventWriter<CombatEvent>,
) {
  let mut rng = rand::thread_rng();
  for mut aoe in qry.iter_mut() {
    aoe.tick_timer.tick(time.delta());
    let between = Uniform::from(aoe.damage_min..(aoe.damage_max + 1.));

    if aoe.tick_timer.just_finished() {
      for victim in &aoe.victims {
        if let Ok((entity, mut c, transform)) = combatant_query.get_mut(*victim) {
          if c.hp <= 0. {
            continue;
          }

          let damage = between.sample(&mut rng);

          c.hp -= damage;

          events.send(CombatEvent::DamageApplied(
            *victim,
            damage,
            transform.translation.xy(),
            c.hp == 0.,
          ));

          if c.hp <= 0. {
            c.hp = 0.;
            events.send(CombatEvent::CombatantKilled(entity, aoe.caster));
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
      }
      _ => {}
    }
  }
}

fn spawn_spell_stuff(
  mut commands: Commands,
  mut actions: EventReader<CombatAction>,
  mut qry: Query<(&mut Spellbook, &Transform)>,
) {
  for action in actions.iter() {
    match action {
      CombatAction::PrepareSpell(entity, spell_type, dir) => {
        if let Ok((mut spellbook, caster_transform)) = qry.get_mut(*entity) {
          let spell = spellbook.spells.get(spell_type).expect("spell not found");
          if let Some(sprite) = &spell.prepare_sprite {
            commands
              .spawn()
              .insert(
                Transform::from_translation(Vec3::new(
                  caster_transform.translation.x,
                  caster_transform.translation.y,
                  crate::z::PLAYER_ATTACK,
                ))
                .with_rotation(Quat::from_rotation_arc(
                  Vec3::X,
                  Vec3::new(dir.x, dir.y, 0.0),
                )),
              )
              .insert(GlobalTransform::default())
              .insert(TimedLife::from_seconds(spell.prepare_duration))
              .with_children(|parent| {
                parent
                  .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite.texture_atlas.clone(),
                    transform: Transform::from_translation(Vec3::new(
                      sprite.translation.x,
                      sprite.translation.y,
                      0.0,
                    )),
                    ..Default::default()
                  })
                  .insert(GlobalTransform::default())
                  .insert(AtlasAnimationDefinition {
                    start: sprite.start_frame,
                    end: sprite.end_frame,
                    fps: if sprite.repeatable {
                      sprite.fps
                    } else {
                      (sprite.end_frame - sprite.start_frame + 1) as f32 / spell.prepare_duration
                    },
                    repeat: sprite.repeatable,
                    random_start: false,
                  })
                  .insert(AtlasAnimation::default());
              });
          }
        } else {
          warn!("spellcaster not found, cannot prepare spell");
        }
      }
      CombatAction::CastSpell(entity, spell_type, dir) => {
        if let Ok((spellbook, caster_transform)) = qry.get(*entity) {
          let spell = spellbook.spells.get(spell_type).expect("spell not found");
          if let Some(sprite) = &spell.cast_sprite {
            commands
              .spawn()
              .insert(
                Transform::from_translation(Vec3::new(
                  caster_transform.translation.x,
                  caster_transform.translation.y,
                  crate::z::PLAYER_ATTACK,
                ))
                .with_rotation(Quat::from_rotation_arc(
                  Vec3::X,
                  Vec3::new(dir.x, dir.y, 0.0),
                )),
              )
              .insert(GlobalTransform::default())
              .insert(TimedLife::from_seconds(spell.cast_duration))
              .insert(RigidBody::Sensor)
              .insert(
                CollisionLayers::none()
                  .with_group(PhysicsLayers::Attacks)
                  .with_mask(PhysicsLayers::Enemies),
              )
              .insert(AreaOfEffect {
                caster: *entity,
                damage_min: spell.damage_min,
                damage_max: spell.damage_max,
                tick_timer: if spell.dot {
                  Timer::from_seconds(spell.damage_tick, true)
                } else {
                  Timer::from_seconds(0.001, false)
                },
                victims: Vec::new(),
              })
              .with_children(|parent| {
                parent
                  .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite.texture_atlas.clone(),
                    transform: Transform::from_translation(Vec3::new(
                      sprite.translation.x,
                      sprite.translation.y,
                      0.0,
                    )),
                    ..Default::default()
                  })
                  .insert(spell.shape.clone())
                  .insert(AtlasAnimationDefinition {
                    start: sprite.start_frame,
                    end: sprite.end_frame,
                    fps: if sprite.repeatable {
                      sprite.fps
                    } else {
                      (sprite.end_frame - sprite.start_frame + 1) as f32 / spell.cast_duration
                    },
                    repeat: sprite.repeatable,
                    random_start: false,
                  })
                  .insert(AtlasAnimation::default());
              });
          }
        } else {
          warn!("spellcaster not found, cannot prepare spell");
        }
      }
      CombatAction::RecoverFromSpell(entity, spell_type, dir) => {
        if let Ok((spellbook, caster_transform)) = qry.get(*entity) {
          let spell = spellbook.spells.get(spell_type).expect("spell not found");
          if let Some(sprite) = &spell.cast_sprite {
            commands
              .spawn()
              .insert(
                Transform::from_translation(Vec3::new(
                  caster_transform.translation.x,
                  caster_transform.translation.y,
                  crate::z::PLAYER_ATTACK,
                ))
                .with_rotation(Quat::from_rotation_arc(
                  Vec3::X,
                  Vec3::new(dir.x, dir.y, 0.0),
                )),
              )
              .insert(GlobalTransform::default())
              .insert(TimedLife::from_seconds(spell.recovery_duration))
              .with_children(|parent| {
                parent
                  .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite.texture_atlas.clone(),
                    transform: Transform::from_translation(Vec3::new(
                      sprite.translation.x,
                      sprite.translation.y,
                      0.0,
                    )),
                    ..Default::default()
                  })
                  .insert(AtlasAnimationDefinition {
                    start: sprite.start_frame,
                    end: sprite.end_frame,
                    fps: if sprite.repeatable {
                      sprite.fps
                    } else {
                      (sprite.end_frame - sprite.start_frame + 1) as f32 / spell.recovery_duration
                    },
                    repeat: sprite.repeatable,
                    random_start: false,
                  })
                  .insert(AtlasAnimation::default());
              });
          }
        } else {
          warn!("spellcaster not found, cannot prepare spell");
        }
      }
      CombatAction::CancelSpell(entity) => {
        // TODO: despawn?
      }
    }
  }
}

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


#[derive(Component)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<CombatEvent>()
      .add_event::<CombatAction>()
      .init_resource::<FlyingTextSettings>()
      .add_startup_system(setup)
      .add_system(show_damage)
      .add_system(spawn_spell_stuff)
      .add_system(find_victims.label("find_victims"))
      .add_system(damage_victims.label("damage_victims").after("find_victims"));
  }
}
