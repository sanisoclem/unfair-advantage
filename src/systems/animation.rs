use bevy::prelude::*;

#[derive(Component)]
pub struct AtlasAnimation {
  pub timer: Timer,
  pub enabled: bool,
  pub start: usize,
  pub end: usize,
}
impl AtlasAnimation {
  pub fn new(fps: f32, start: usize, end: usize, enabled: bool) -> Self {
    Self {
      timer: Timer::from_seconds(1. / fps, true),
      start,
      end,
      enabled,
    }
  }
}

fn animate_sprites(
  time: Res<Time>,
  mut query: Query<(
    &mut AtlasAnimation,
    &mut TextureAtlasSprite
  )>,
) {
  for (mut animation, mut sprite) in query.iter_mut() {
    if animation.enabled {
      animation.timer.tick(time.delta());
      if animation.timer.just_finished() {
        sprite.index = animation.start + ((sprite.index + 1 - animation.start) % (animation.end - animation.start + 1));
      }
    }
  }
}

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(animate_sprites);
  }
}
