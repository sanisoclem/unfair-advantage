use bevy::prelude::*;
use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
    World,
    Player,
    Enemies,
}

#[derive(Component)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(heron::prelude::PhysicsPlugin::default())
      .add_system(log_collisions)
      .insert_resource(Gravity::from(Vec3::new(0.0, 0.0, 0.0))) ;
  }
}

fn log_collisions(mut events: EventReader<CollisionEvent>) {
  for event in events.iter() {
      match event {
          CollisionEvent::Started(d1, d2) => {
              //println!("Collision started between {:?} and {:?}", d1, d2)
          }
          CollisionEvent::Stopped(d1, d2) => {
              //println!("Collision stopped between {:?} and {:?}", d1, d2)
          }
      }
  }
}