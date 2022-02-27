use bevy::utils::Duration;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

#[derive(Component)]
pub struct AtlasAnimation {
  pub timer: Timer,
  pub start_frame: usize,
  pub enabled: bool,
  pub complete: bool,
}
impl Default for AtlasAnimation {
  fn default() -> Self {
    AtlasAnimation {
      timer: Timer::from_seconds(0.0, false),
      enabled: false,
      start_frame: 0,
      complete: false,
    }
  }
}

#[derive(Component, Clone)]
pub struct AtlasAnimationDefinition {
  pub start: usize,
  pub end: usize,
  pub fps: f32,
  pub repeat: bool,
  pub random_start: bool
}

#[derive(Component)]
pub struct CustomAnimation<T1, T2> {
  pub ease: fn(Duration, Mut<T1>, Mut<T2>),
}

pub fn animate_custom<T1: Component, T2: Component>(
  time: Res<Time>,
  mut qry: Query<(&CustomAnimation<T1, T2>, &mut T1, &mut T2)>,
) {
  for (anim, c1, c2) in qry.iter_mut() {
    (anim.ease)(time.delta(), c1, c2);
  }
}

#[derive(Component)]
pub struct TimedLife {
  pub timer: Timer,
}
impl TimedLife {
  pub fn from_seconds(seconds: f32) -> Self {
    TimedLife {
      timer: Timer::from_seconds(seconds, false),
    }
  }
}

fn despawn_timed_lives(mut commands: Commands, time: Res<Time>, mut qry: Query<(Entity, &mut TimedLife)>) {
  for (entity, mut life) in qry.iter_mut() {
    life.timer.tick(time.delta());
    if life.timer.just_finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn init_atlas_animation(
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
      anim.timer = Timer::from_seconds(1. / def.fps, true);
      anim.enabled = true;

      anim.start_frame = if def.random_start {
        let between = Uniform::from(def.start..(def.end + 1));
        between.sample(&mut rng)
      } else {
        0
      };

      sprite.index = anim.start_frame;
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

        if !def.repeat && sprite.index == animation.start_frame {
          animation.enabled = false;
          animation.complete = true;
        }
      }
    }
  }
}

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(init_atlas_animation)
      .add_system(animate_sprites)
      .add_system(despawn_timed_lives);
  }
}
