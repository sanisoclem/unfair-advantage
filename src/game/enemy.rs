use crate::systems::AtlasAnimation;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::{fmt::Debug, hash::Hash};

pub enum EnemyCommand {
  Spawn(EnemyType, Vec3),
  DespawnAll,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum EnemyType {
  Slime,
  Goblin,
  Eye,
}

pub struct AtlasAnimationDefinition {
  pub start: usize,
  pub end: usize,
  pub fps: f32,
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
            .insert(AtlasAnimation::new(def.idle.fps, def.idle.start, def.idle.end, true))
            .insert(Enemy);
            info!("spawned enemy {:?}", enemy_type);
        } else {
          warn!("Enemy type not found: {:?}", enemy_type);
        }
      }
      EnemyCommand::DespawnAll => {
        //TODO: despawn all enemies
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
      scale: Vec3::splat(3.0),
      idle: AtlasAnimationDefinition {
        start: 84,
        end: 89,
        fps: 10.,
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
