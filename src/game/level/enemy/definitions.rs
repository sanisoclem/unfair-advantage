use crate::systems::AtlasAnimationDefinition;
use bevy::{prelude::*, utils::HashMap};

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum EnemyType {
  Slime,
  Goblin,
  Boss,
  Eye,
}

pub struct EnemyDefinition {
  pub texture_atlas: Handle<TextureAtlas>,
  pub idle: AtlasAnimationDefinition,
  pub death: AtlasAnimationDefinition,
  pub max_hp: f32,
  pub fodder: bool,
}

pub struct EnemyDictionary {
  pub enemies: HashMap<EnemyType, EnemyDefinition>,
}

impl FromWorld for EnemyDictionary {
  fn from_world(world: &mut World) -> Self {
    let (tx_peon, tx_boss) = {
      let asset_server = world
        .get_resource::<AssetServer>()
        .expect("should find asset server");
      (
        asset_server.load("full spritesheet2.png"),
        asset_server.load("boss.png"),
      )
    };

    let mut texture_atlases = world
      .get_resource_mut::<Assets<TextureAtlas>>()
      .expect("should find texture atlases");
    let texture_atlas = TextureAtlas::from_grid(tx_peon, Vec2::new(16.0, 16.0), 28, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut enemies = HashMap::default();
    enemies.insert(
      EnemyType::Slime,
      EnemyDefinition {
        max_hp: 10.,
        fodder: true,
        texture_atlas: texture_atlas_handle.clone(),
        idle: AtlasAnimationDefinition {
          start: 84,
          end: 89,
          fps: 10.,
          repeat: true,
          random_start: true,
          repeat_from: None,
        },
        death: AtlasAnimationDefinition {
          start: 84,
          end: 103,
          fps: 10.,
          repeat: false,
          random_start: false,
          repeat_from: None,
        },
      },
    );
    enemies.insert(
      EnemyType::Goblin,
      EnemyDefinition {
        max_hp: 10.,
        fodder: true,
        texture_atlas: texture_atlas_handle.clone(),
        idle: AtlasAnimationDefinition {
          start: 28,
          end: 33,
          fps: 10.,
          repeat: true,
          random_start: true,
          repeat_from: None,
        },
        death: AtlasAnimationDefinition {
          start: 28,
          end: 41,
          fps: 10.,
          repeat: false,
          random_start: false,
          repeat_from: None,
        },
      },
    );

    enemies.insert(
      EnemyType::Eye,
      EnemyDefinition {
        max_hp: 10.,
        fodder: true,
        texture_atlas: texture_atlas_handle.clone(),
        idle: AtlasAnimationDefinition {
          start: 0,
          end: 3,
          fps: 10.,
          repeat: true,
          random_start: true,
          repeat_from: None,
        },
        death: AtlasAnimationDefinition {
          start: 0,
          end: 11,
          fps: 10.,
          repeat: false,
          random_start: false,
          repeat_from: None,
        },
      },
    );

    let texture_atlas = TextureAtlas::from_grid(tx_boss, Vec2::new(256.0, 256.0), 24, 17);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    enemies.insert(
      EnemyType::Boss,
      EnemyDefinition {
        max_hp: 10.,
        fodder: true,
        texture_atlas: texture_atlas_handle.clone(),
        idle: AtlasAnimationDefinition {
          start: 336,
          end: 336 + 11,
          fps: 10.,
          repeat: true,
          repeat_from: None,
          random_start: true,
        },
        death: AtlasAnimationDefinition {
          start: 0,
          end: 23,
          fps: 10.,
          repeat: false,
          random_start: false,
          repeat_from: None,
        },
      },
    );

    EnemyDictionary { enemies }
  }
}
