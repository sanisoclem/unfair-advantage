use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
  pub target: Option<Vec2>,
  pub speed: f32,
  pub enabled: bool
}

fn movement(
  time: Res<Time>,
  mut qry: Query<(
    &mut Movement,
    &mut Transform
  )>,
) {
  for (mut mov, mut transform) in qry.iter_mut() {
    if !mov.enabled || mov.speed == 0.0 {
      continue;
    }

    if let Some(target) = mov.target {
      let diff: Vec3 = transform.translation -  Vec3::from((target, 0.));
      let factor = mov.speed * time.delta_seconds();

      if diff.length() <= factor {
        transform.translation = Vec3::from((target, 0.));
        mov.target = None;
      } else {
        transform.translation -= diff.normalize() * factor;
      }
    }
  }
}

#[derive(Component)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(movement);
  }
}
