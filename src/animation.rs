use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
  timer: Timer,
  enabled: bool,
}
impl Animation {
  pub fn new(fps: f32, enabled: bool) -> Self {
    Self {
      timer: Timer::from_seconds(1. / fps, true),
      enabled,
    }
  }
  pub fn enable(&mut self, enabled: bool) {
    self.enabled = enabled;
  }
}

fn animate_sprites(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut query: Query<(
    &mut Animation,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
) {
  for (mut animation, mut sprite, texture_atlas_handle) in query.iter_mut() {
    if animation.enabled {
      animation.timer.tick(time.delta());
      if animation.timer.just_finished() {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
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
