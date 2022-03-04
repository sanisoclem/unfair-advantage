use bevy::prelude::*;
use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
  World,
  Player,
  Enemies,
  Attacks,
  AttackDead,
  Corpses,
  MovementSensor,
}

#[derive(Component)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(heron::prelude::PhysicsPlugin::default())
      //.add_system(log_collisions)
      .insert_resource(Gravity::from(Vec3::new(0.0, 0.0, 0.0)));
  }
}
