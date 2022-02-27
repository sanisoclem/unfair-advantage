use crate::systems::{AtlasAnimation, PhysicsLayers, AtlasAnimationDefinition};
use bevy::prelude::*;
use heron::prelude::*;
use bevy::utils::HashMap;
use std::{fmt::Debug, hash::Hash};

pub enum EnemyCommand {
  Spawn(EnemyType, Vec3),
  //SpawnBatch(Vec<(EnemyType, Vec3)>),
  //DespawnAll,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum EnemyType {
  Slime,
  //Goblin,
  //Eye,
}

pub struct EnemyDefinition {
  pub texture_atlas: Handle<TextureAtlas>,
  pub scale: Vec3,
  pub idle: AtlasAnimationDefinition,
}

#[derive(Default)]
pub struct EnemyDictionary {
  pub enemies: HashMap<EnemyType, EnemyDefinition>,
}

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemies(
  mut cmds: EventReader<EnemyCommand>,
  mut commands: Commands,
  enemy_dict: Res<EnemyDictionary>,
) {
  for evt in cmds.iter() {
    match evt {
      EnemyCommand::Spawn(enemy_type, pos) => {
        if let Some(def) = enemy_dict.enemies.get(enemy_type) {
          commands
            .spawn_bundle(SpriteSheetBundle {
              texture_atlas: def.texture_atlas.clone(),
              transform: Transform::from_scale(def.scale).with_translation(pos.clone()),
              ..Default::default()
            })
            .insert(def.idle.clone())
            .insert(AtlasAnimation::default())
            .insert(Enemy)//physics
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere {  radius: 7. })
            //.insert(Velocity::from_linear(Vec3::default()))
            //.insert(Acceleration::from_linear(Vec3::default()))
            .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
            .insert(RotationConstraints::lock())
            .insert(Damping::from_linear(10.0))
            .insert(CollisionLayers::none().with_group(PhysicsLayers::Enemies).with_mask(PhysicsLayers::Player).with_mask(PhysicsLayers::Enemies).with_mask(PhysicsLayers::World));
            info!("spawned enemy {:?}", enemy_type);
        } else {
          warn!("Enemy type not found: {:?}", enemy_type);
        }
      }
      _ => {
        warn!("not implemented");
      }
    }
  }
}

fn setup(
  asset_server: Res<AssetServer>,
  mut enemy_dict: ResMut<EnemyDictionary>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("pack2/full spritesheet.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 28, 7);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  let mut enemies = HashMap::default();

  enemies.insert(
    EnemyType::Slime,
    EnemyDefinition {
      texture_atlas: texture_atlas_handle.clone(),
      scale: Vec3::splat(1.0),
      idle: AtlasAnimationDefinition {
        start: 84,
        end: 89,
        fps: 10.,
        repeat: true
      },
    },
  );

  enemy_dict.enemies = enemies;
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(EnemyDictionary::default())
      .add_event::<EnemyCommand>()
      .add_startup_system(setup)
      .add_system(spawn_enemies);
  }
}