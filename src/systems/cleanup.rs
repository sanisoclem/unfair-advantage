use bevy::prelude::*;

pub fn cleanup_system<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in to_despawn.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
