use super::{
  generator::{Level, TileType, WallType},
  settings::LevelSettings,
  LevelLoader, LevelState, LevelTag,
};
use crate::systems::PhysicsLayers;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use heron::prelude::*;

// pub fn despawn_player(mut player_state: ResMut<State<PlayerState>>) {
//   player_state
//     .set(PlayerState::Despawned)
//     .expect("set player state should always succeed");
// }

// pub fn spawn_player(mut player_state: ResMut<State<PlayerState>>) {
//   player_state
//     .set(PlayerState::Active)
//     .expect("set player state should always succeed");
// }

pub fn generate_level(
  mut commands: Commands,
  mut map_query: MapQuery,
  mut level: ResMut<Level>,
  settings: Res<LevelSettings<WallType, TileType>>,
) {
  *level = Level::generate_random(
    settings.map_size.0 * settings.chunk_size.0,
    settings.map_size.1 * settings.chunk_size.1,
  );

  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);
  let layer_settings = LayerSettings::new(
    MapSize(settings.map_size.0, settings.map_size.1),
    ChunkSize(settings.chunk_size.0, settings.chunk_size.1),
    TileSize(settings.tile_size.x, settings.tile_size.y),
    TextureSize(settings.tilemap_size.x, settings.tilemap_size.y),
  );

  let (mut layer1_builder, layer_0_entity) =
    LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, 0u16);
  let (mut layer2_builder, layer_1_entity) =
    LayerBuilder::<TileBundle>::new(&mut commands, layer_settings.clone(), 0u16, 1u16);

  for x in 0..level.width {
    for y in 0..level.height {
      let position = TilePos(x, y);
      if let Some(tile_index) = settings.get_floor_tile(level.get(x as i32, y as i32)) {
        layer1_builder
          .set_tile(
            position,
            Tile {
              texture_index: tile_index,
              ..Default::default()
            }
            .into(),
          )
          .expect("should succeed");
      }
      if let Some(wall_index) = settings.get_wall_tile(level.get_wall(x as i32, y as i32)) {
        layer2_builder
          .set_tile(
            position,
            Tile {
              texture_index: wall_index,
              ..Default::default()
            }
            .into(),
          )
          .expect("should succeed");
      }
    }
  }

  map_query.build_layer(&mut commands, layer1_builder, settings.tilemap.clone());
  map_query.build_layer(&mut commands, layer2_builder, settings.tilemap.clone());
  map.add_layer(&mut commands, 0u16, layer_0_entity);
  map.add_layer(&mut commands, 1u16, layer_1_entity);

  commands
    .entity(map_entity)
    .insert(map)
    .insert(LevelTag)
    .insert(Transform::from_xyz(0.0, 0.0, crate::z::GROUND))
    .insert(GlobalTransform::default());

  // spawn collission shapes
  for rect in level.collission_shapes.iter() {
    commands
      .spawn()
      .insert(Transform::from_translation(Vec3::new(
        (rect.x as f32 * layer_settings.tile_size.0
          + layer_settings.tile_size.0 * (rect.width as f32 / 2.))
          / 3.,
        (rect.y as f32 * layer_settings.tile_size.1
          + layer_settings.tile_size.1 * (rect.height as f32 / 2.))
          / 3.,
        crate::z::WALLS,
      )))
      .insert(LevelTag)
      .insert(GlobalTransform::default())
      .insert(RigidBody::Static)
      .insert(
        CollisionLayers::none()
          .with_group(PhysicsLayers::World)
          .with_mask(PhysicsLayers::Enemies)
          .with_mask(PhysicsLayers::Attacks)
          .with_mask(PhysicsLayers::Player)
          .with_mask(PhysicsLayers::Corpses)
      )
      .insert(CollisionShape::Cuboid {
        half_extends: Vec3::new(
          rect.width as f32 * layer_settings.tile_size.0 / 6.,
          rect.height as f32 * layer_settings.tile_size.1 / 6.,
          0.0,
        ),
        border_radius: Some(0.1),
      });
  }

  commands.spawn().insert(LevelTag).insert(LevelLoader {
    timer: Timer::from_seconds(1.0, false),
  });

  commands
    .spawn()
    .insert(LevelTag)
    .insert(Transform::from_translation(Vec3::new(
      level.exit_point.x as f32 * layer_settings.tile_size.0 + layer_settings.tile_size.0 / 2.,
      level.exit_point.y as f32 * layer_settings.tile_size.0 + layer_settings.tile_size.0 / 2.,
      0.,
    )))
    .insert(GlobalTransform::default())
    .insert(RigidBody::Sensor)
    .insert(
      CollisionLayers::none()
        .with_group(PhysicsLayers::Exit)
        .with_mask(PhysicsLayers::Player),
    )
    .insert(CollisionShape::Cuboid {
      half_extends: Vec3::new(settings.tile_size.x / 2., settings.tile_size.y / 2., 0.),
      border_radius: None,
    });
}

pub fn load_complete(
  time: Res<Time>,
  mut qry: Query<&mut LevelLoader>,
  mut level_state: ResMut<State<LevelState>>,
) {
  for mut loader in (&mut qry).iter_mut() {
    loader.timer.tick(time.delta());
    if loader.timer.just_finished() {
      level_state
        .set(LevelState::Loaded)
        .expect("set level state should always succeed");
    }
  }
}

pub fn check_level_complete(
  mut level_state: ResMut<State<LevelState>>,
  mut events: EventReader<CollisionEvent>,
) {
  events
    .iter()
    .filter_map(|event| {
      let (entity_1, entity_2) = event.rigid_body_entities();
      let (layers_1, layers_2) = event.collision_layers();
      if layers_1.contains_group(PhysicsLayers::Player)
        && layers_2.contains_group(PhysicsLayers::Exit)
      {
        Some((entity_2, entity_1, event))
      } else if layers_2.contains_group(PhysicsLayers::Player)
        && layers_1.contains_group(PhysicsLayers::Exit)
      {
        Some((entity_1, entity_2, event))
      } else {
        None
      }
    })
    .for_each(|(_exit, _player, _e)| {
      level_state
        .set(LevelState::LevelComplete)
        .expect("set level state should always succeed");
    });
}
