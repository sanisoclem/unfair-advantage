use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
  pub target: Option<Vec2>,
  pub last_direction: Vec2,
  pub speed: f32,
  pub enabled: bool,
}

fn movement(time: Res<Time>, mut qry: Query<(&mut Movement, &mut Transform)>) {
  for (mut mov, mut transform) in qry.iter_mut() {
    if !mov.enabled || mov.speed == 0.0 {
      continue;
    }

    if let Some(target) = mov.target {
      let diff = transform.translation.truncate() - target;
      let factor = mov.speed * time.delta_seconds();
      let z = transform.translation.z;

      mov.last_direction = diff.normalize() * -1.;

      if diff.length() <= factor {
        transform.translation = Vec3::from((target, z));
        mov.target = None;
      } else {
        transform.translation -= Vec3::from((diff, 0.)).normalize() * factor;
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
