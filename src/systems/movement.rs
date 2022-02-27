use bevy::{math::Vec3Swizzles, prelude::*};
use heron::{PhysicMaterial, Velocity};

#[derive(Component)]
pub struct Movement {
  pub target: Option<Vec2>,
  pub last_direction: Vec2,
  pub speed: f32,
  pub enabled: bool,
}

fn movement(time: Res<Time>, mut qry: Query<(&mut Movement, &mut Transform), Without<Velocity>>) {
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

fn movement_phys(
  time: Res<Time>,
  mut qry: Query<(&mut Movement, &Transform, &PhysicMaterial, &mut Velocity)>,
) {
  for (mut mov, transform, mat, mut v) in qry.iter_mut() {
    if !mov.enabled || mov.speed == 0.0 {
      continue;
    }

    if let Some(target) = mov.target {
      let diff = target - transform.translation.xy();
      if diff.length() < 1.0 {
        mov.target = None;
        v.linear = Vec3::default();
        continue;
      }

      let desired_v = diff.normalize() * mov.speed;

      v.linear = v.linear.lerp(
        Vec3::from((desired_v, 0.)),
        time.delta_seconds() / mat.density * 100.,
      );
    }
  }
}

#[derive(Component)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(movement).add_system(movement_phys);
  }
}
