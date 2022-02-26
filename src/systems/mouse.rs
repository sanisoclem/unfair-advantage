use super::MainCamera;
use bevy::prelude::*;

#[derive(Default)]
pub struct MouseInfo {
  pub screen_pos: Vec2,
  pub world_pos3: Vec3,
  pub world_pos2: Vec2,
}

fn set_mouse_info(
  mut mouse_info: ResMut<MouseInfo>,
  windows: Res<Windows>,
  q_camera: Query<(&Camera, &GlobalTransform, &MainCamera)>,
) {
  let window = windows.get_primary().expect("should have a primary window");
  for (camera, camera_transform, _) in q_camera.iter() {
    if let Some(pos) = window.cursor_position() {
      // get the size of the window
      let window_size = Vec2::new(window.width() as f32, window.height() as f32);

      // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
      let ndc = (pos / window_size) * 2.0 - Vec2::ONE;

      // matrix for undoing the projection and camera transform
      let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

      // use it to convert ndc to world-space coordinates
      let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

      mouse_info.world_pos3 = world_pos;
      mouse_info.world_pos2 = world_pos.truncate();
      mouse_info.screen_pos = pos;
    }
  }
}

#[derive(Component)]
pub struct MousePlugin;

impl Plugin for MousePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(MouseInfo::default())
      .add_system(set_mouse_info);
  }
}
