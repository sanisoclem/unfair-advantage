use bevy::prelude::*;

#[derive(Component)]
pub struct Combatant {
  pub hp: f32,
  pub hp_max: f32
}

#[derive(Component)]
pub struct AreaOfEffect {
  pub dps: f32,
  pub timer: Timer,
  pub knockback: f32,
}

fn fight(
  time: Res<Time>,
  mut query: Query<(
    Entity,
    &Collider,
    &Transform,
    &mut Combatant,
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
