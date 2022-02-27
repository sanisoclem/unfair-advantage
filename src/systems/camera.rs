use crate::systems::Movement;
use bevy::{math::Vec3Swizzles, prelude::*};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraTarget;

fn setup(mut commands: Commands) {
  let mut cam_bundle = OrthographicCameraBundle::new_2d();
  cam_bundle.transform = cam_bundle.transform.with_scale(Vec3::new(0.25, 0.25, 1.0));

  commands
    .spawn_bundle(cam_bundle)
    .insert(MainCamera)
    .insert(Movement {
      last_direction: Vec2::default(),
      speed: 600.0,
      enabled: true,
      target: None,
    });
}

fn camera_system(
  mut qry: Query<(&MainCamera, &mut Movement)>,
  qry_target: Query<(Without<MainCamera>, &CameraTarget, &Transform)>,
) {
  if let Ok((_, _, target_transform)) = qry_target.get_single() {
    for (_, mut mov) in qry.iter_mut() {
      mov.target = Some(target_transform.translation.xy());
    }
  }
}

#[derive(Component)]
pub struct TopDownCameraPlugin;

impl Plugin for TopDownCameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup).add_system(camera_system);
  }
}
