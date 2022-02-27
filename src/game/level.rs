use super::{
  enemy::{EnemyCommand, EnemyType},
  player::PlayerState,
};
use crate::systems::cleanup_system;
use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelState {
  Disabled,
  TestLevel,
  //ProceduralLevel(String),
}

#[derive(Component)]
pub struct LevelTag;

fn setup_test_level(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut player_state: ResMut<State<PlayerState>>,
  mut map_query: MapQuery,
  mut enemy_cmd: EventWriter<EnemyCommand>,
) {
  let texture_handle = asset_server.load("pack1/TX Tileset Grass.png");
  let map_size = MapSize(20, 20);

  let layer_settings = LayerSettings::new(
    map_size,
    ChunkSize(32, 32),
    TileSize(16.0, 16.0),
    TextureSize(256.0, 256.0),
  );

  let (mut layer_builder, layer_0_entity) =
    LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, 0u16);

  layer_builder.set_all(
    Tile {
      texture_index: 75,
      ..Default::default()
    }
    .into(),
  );

  map_query.build_layer(&mut commands, layer_builder, texture_handle);

  // Create map entity and component:
  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);

  // Required to keep track of layers for a map internally.
  map.add_layer(&mut commands, 0u16, layer_0_entity);

  // Spawn Map
  // Required in order to use map_query to retrieve layers/tiles.
  commands
    .entity(map_entity)
    .insert(map)
    .insert(Transform::from_xyz(-5120.0, -5120.0, crate::z::GROUND).with_scale(Vec3::splat(3.0)))
    .insert(GlobalTransform::default());

  player_state
    .set(PlayerState::Active)
    .expect("set player state should always succeed");

  for i in 20..70 {
    let r = 10;
    let sz = 16.;
    let pos = Vec2::new(((i % r) - r/2) as f32, (i/r) as f32);
    enemy_cmd.send(EnemyCommand::Spawn(
      EnemyType::Slime,
      pos * sz,
    ));
  }

  for i in 20..70 {
    let r = 10;
    let sz = 16.;
    let pos = Vec2::new(((i % r) - r/2 - 11) as f32, (i/r) as f32);
    enemy_cmd.send(EnemyCommand::Spawn(
      EnemyType::Goblin,
      pos * sz,
    ));
  }

  for i in 20..70 {
    let r = 10;
    let sz = 16.;
    let pos = Vec2::new(((i % r) - r/2 + 11) as f32, (i/r) as f32);
    enemy_cmd.send(EnemyCommand::Spawn(
      EnemyType::Goblin,
      pos * sz,
    ));
  }
}

fn teardown_level(mut player_state: ResMut<State<PlayerState>>) {
  player_state
    .set(PlayerState::Despawned)
    .expect("set player state should always succeed");
}

pub fn set_texture_filters_to_nearest(
  mut texture_events: EventReader<AssetEvent<Image>>,
  mut textures: ResMut<Assets<Image>>,
) {
  // quick and dirty, run this for all textures anytime a texture is created.
  for event in texture_events.iter() {
    match event {
      AssetEvent::Created { handle } => {
        if let Some(mut texture) = textures.get_mut(handle) {
          texture.texture_descriptor.usage =
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST;
        }
      }
      _ => (),
    }
  }
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(TilemapPlugin)
      .add_state(LevelState::Disabled)
      .add_system(set_texture_filters_to_nearest)
      .add_system_set(SystemSet::on_enter(LevelState::TestLevel).with_system(setup_test_level))
      .add_system_set(
        SystemSet::on_exit(LevelState::TestLevel)
          .with_system(teardown_level)
          .with_system(cleanup_system::<LevelTag>),
      );
  }
}
