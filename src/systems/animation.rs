use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

#[derive(Component)]
pub struct AtlasAnimation {
  pub timer: Timer,
  pub enabled: bool,
}
impl Default for AtlasAnimation {
  fn default() -> Self {
    AtlasAnimation {
      timer: Timer::from_seconds(0.0, false),
      enabled: false,
    }
  }
}

#[derive(Component, Clone)]
pub struct AtlasAnimationDefinition {
  pub start: usize,
  pub end: usize,
  pub fps: f32,
  pub repeat: bool,
}

fn create_animation(
  mut qry: Query<
    (
      &mut AtlasAnimation,
      &AtlasAnimationDefinition,
      &mut TextureAtlasSprite,
    ),
    Or<(
      Changed<AtlasAnimationDefinition>,
      Added<AtlasAnimationDefinition>,
    )>,
  >,
) {
  if !qry.is_empty() {
    let mut rng = rand::thread_rng();

    for (mut anim, def, mut sprite) in qry.iter_mut() {
      let between = Uniform::from(def.start..(def.end + 1));
      anim.timer = Timer::from_seconds(1. / def.fps, def.repeat);
      anim.enabled = true;
      // start on random sprite
      sprite.index = between.sample(&mut rng);
    }
  }
}

fn animate_sprites(
  time: Res<Time>,
  mut query: Query<(
    &mut AtlasAnimation,
    &AtlasAnimationDefinition,
    &mut TextureAtlasSprite,
  )>,
) {
  for (mut animation, def, mut sprite) in query.iter_mut() {
    if animation.enabled {
      animation.timer.tick(time.delta());
      if animation.timer.just_finished() {
        sprite.index = def.start + ((sprite.index + 1 - def.start) % (def.end - def.start + 1));
      }
    }
  }
}

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(create_animation).add_system(animate_sprites);
  }
}
