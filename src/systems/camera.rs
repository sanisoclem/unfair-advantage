use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
  commands
    .spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(MainCamera);
}

#[derive(Component)]
pub struct TopDownCameraPlugin;

impl Plugin for TopDownCameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup);
  }
}
