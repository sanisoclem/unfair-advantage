use crate::systems::Movement;
use super::{
  enemy::{EnemyCommand, EnemyType},
  player::PlayerState,
};
use crate::systems::{cleanup_system, PhysicsLayers};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use heron::prelude::*;
use std::{fmt::Debug, hash::Hash};

use bevy_prototype_lyon::prelude::*;

mod generator;
mod maps2;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelState {
  Disabled,
  TestLevel,
  // ProceduralLevel(String),
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
  let texture_handle = asset_server.load("pack2/full tilemap.png");
  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);

  let layer_settings = LayerSettings::new(
      MapSize(1, 1),
      ChunkSize(64, 64),
      TileSize(16.0, 16.0),
      TextureSize(144.0, 128.0),
  );

  let (mut layer_builder, layer_0_entity) =
    LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, 0u16);

  let level = maps2::Level::generate_random(layer_settings.map_size.0 * layer_settings.chunk_size.0, layer_settings.map_size.1 * layer_settings.chunk_size.1);


  for x in 0..level.width {
    for y in 0..level.height {
      let position = TilePos(x, y);
      let tile_index = match level.get(x, y) {
        maps2::TileType::Floor => Some(23),
        maps2::TileType::Wall => Some(48),
        maps2::TileType::Nothing => None,
      };

      match tile_index {
        Some(tile_index) => {
          //info!("found tile {:?}: {:?}", position, tile_index);
          layer_builder.set_tile(
            position,
            Tile {
                texture_index: tile_index,
                ..Default::default()
            }
            .into()).expect("should succeed");
        }
        None => {}
      }
    }
  }

  map_query.build_layer(&mut commands, layer_builder, texture_handle);
  map.add_layer(&mut commands, 0u16, layer_0_entity);

  commands
    .entity(map_entity)
    .insert(map)
    .insert(Transform::from_xyz(0.0, 0.0, crate::z::GROUND))
    .insert(GlobalTransform::default());

  player_state
    .set(PlayerState::Active)
    .expect("set player state should always succeed");

  // for i in 20..44 {
  //   let r = 10;
  //   let sz = 16.;
  //   let pos = Vec2::new(((i % r) - r / 2) as f32, (i / r) as f32);
  //   enemy_cmd.send(EnemyCommand::Spawn(EnemyType::Slime, pos * sz));
  // }

  // for i in 20..200 {
  //   let r = 10;
  //   let sz = 16.;
  //   let pos = Vec2::new(((i % r) - r / 2 - 11) as f32, (i / r) as f32);
  //   enemy_cmd.send(EnemyCommand::Spawn(EnemyType::Goblin, pos * sz));
  // }

  // for i in 20..200 {
  //   let r = 10;
  //   let sz = 16.;
  //   let pos = Vec2::new(((i % r) - r / 2 + 11) as f32, (i / r) as f32);
  //   enemy_cmd.send(EnemyCommand::Spawn(EnemyType::Goblin, pos * sz));
  // }

  // enemy_cmd.send(EnemyCommand::Spawn(EnemyType::Boss, Vec2::new(0., -300.)));

  // for (p1, p2) in generator::generate_rooms() {
  //   let scale = 500.;
  //   let p1s = p1 * scale;
  //   let p2s = p2 * scale;

  //   let shape = shapes::Rectangle {
  //     extents: p2s,
  //     origin: RectangleOrigin::Center,
  //   };

  //   commands
  //     .spawn_bundle(GeometryBuilder::build_as(
  //       &shape,
  //       DrawMode::Outlined {
  //         fill_mode: FillMode::color(Color::CYAN),
  //         outline_mode: StrokeMode::new(Color::BLACK, 10.0),
  //       },
  //       Transform::from_translation(Vec3::from((p1s, 30.))),
  //     ))
  //     .insert(RotationConstraints::lock())
  //     .insert(RigidBody::Dynamic)
  //     .insert(
  //       CollisionLayers::none()
  //         .with_group(PhysicsLayers::Enemies)
  //         .with_mask(PhysicsLayers::Enemies),
  //     )
  //     .insert(Movement {
  //       target: Some(Vec2::new(0., 0.)),
  //       speed: 50.,
  //       enabled: true
  //     })
  //     .insert(CollisionShape::Cuboid {
  //       half_extends: Vec3::new(p2s.x / 2., p2s.y / 2., 0.),
  //       border_radius: None,
  //     })
  //     .insert(LevelRoom { size: p2s });
  // }
}

#[derive(Component, Clone)]
pub struct LevelRoom {
  pub size: Vec2,
}

fn test(
  time: Res<Time>,
  mut qry: Query<(
    &bevy_prototype_lyon::render::Shape,
    &mut Transform,
    &LevelRoom,
  )>,
) {
  let neighbors = qry
    .iter()
    .map(|(_, t, r)| (t.translation.xy(), r.clone()))
    .collect::<Vec<_>>();
  for (_, mut transform, r1) in qry.iter_mut() {
    let mut separation = Vec2::new(0., 0.);
    let mut cohesion = Vec2::new(0., 0.);
    let t1 = transform.translation.xy();
    let mut neighbor_count = 0;

    for (t2, r2) in neighbors.iter() {
      let diff = *t2 - t1;
      let min_dist = (r1.size + r2.size).length();
      if &t1 == t2 {
        continue;
      }
      separation += *t2 - t1;
      cohesion += *t2;
      neighbor_count += 1;
    }
    if (neighbor_count > 0) {
      cohesion = ((cohesion / neighbor_count as f32) - t1).normalize_or_zero();
      separation = (separation * -1. / neighbor_count as f32).normalize_or_zero();
      let s3 = Vec3::from((cohesion * 0.9 + separation, 0.));
      //transform.translation += s3 * time.delta_seconds() * 100.;
    }

    //info!("transform {:?}", s3);
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
      .add_system(test)
      .add_system_set(SystemSet::on_enter(LevelState::TestLevel).with_system(setup_test_level))
      .add_system_set(
        SystemSet::on_exit(LevelState::TestLevel)
          .with_system(teardown_level)
          .with_system(cleanup_system::<LevelTag>),
      );
  }
}
