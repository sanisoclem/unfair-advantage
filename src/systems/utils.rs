use bevy::{prelude::*, render::render_resource::TextureUsages};

pub fn set_texture_filters_to_nearest(
  mut texture_events: EventReader<AssetEvent<Image>>,
  mut textures: ResMut<Assets<Image>>,
) {
  // quick and dirty, run this for all textures anytime a texture is created.
  for event in texture_events.iter() {
    match event {
      AssetEvent::Created { handle } => {
        if let Some(mut texture) = textures.get_mut(handle) {
          texture.texture_descriptor.usage =
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST;
        }
      }
      _ => (),
    }
  }
}

pub fn cleanup_system<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in to_despawn.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
