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

fn camera_system_initial_focus(
  mut qry: Query<(&MainCamera, &mut Transform)>,
  qry_target: Query<(Without<MainCamera>, &CameraTarget, &Transform), Added<CameraTarget>>,
) {
  if let Ok((_, _, target_transform)) = qry_target.get_single() {
    for (_, mut cam_transform) in qry.iter_mut() {
      cam_transform.translation.x = target_transform.translation.x;
      cam_transform.translation.y = target_transform.translation.y;
    }
  }
}

#[derive(Component)]
pub struct TopDownCameraPlugin;

impl Plugin for TopDownCameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup).add_system(camera_system)
    .add_system(camera_system_initial_focus);
  }
}
