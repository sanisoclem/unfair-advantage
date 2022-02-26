use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationComponent(pub Timer);

fn animate_sprites(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut query: Query<(
    &mut AnimationComponent,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
) {
  for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
      sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
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