use bevy::utils::Duration;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

#[derive(Component)]
pub struct AtlasAnimation {
  pub timer: Timer,
  pub start_frame: usize,
  pub enabled: bool,
}
impl Default for AtlasAnimation {
  fn default() -> Self {
    AtlasAnimation {
      timer: Timer::from_seconds(0.0, false),
      enabled: false,
      start_frame: 0,
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
pub struct CustomDiscreteAnimation<T, TData> {
  pub timer: Timer,
  pub data: TData,
  pub ease: fn(&TData, Mut<T>) -> TData,
}

pub fn animate_custom_discrete<T: Component, TData: Component>(
  time: Res<Time>,
  mut qry: Query<(&mut CustomDiscreteAnimation<T, TData>, &mut T)>,
) {
  for (mut anim, comp) in qry.iter_mut() {
    anim.timer.tick(time.delta());
    if anim.timer.just_finished() {
      anim.data = (anim.ease)(&anim.data, comp);
    }
  }
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
      .add_system(create_animation)
      .add_system(animate_sprites);
  }
}
