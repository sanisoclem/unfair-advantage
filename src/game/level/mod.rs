use crate::systems::cleanup_system;
use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_tilemap::prelude::*;
use std::{fmt::Debug, hash::Hash};

use generator::{TileType, WallType};
use settings::LevelSettings;
use systems::*;

pub mod camera;
pub mod complete;
pub mod enemy;
pub mod generator;
pub mod loading;
pub mod player;
pub mod settings;
pub mod systems;
pub mod ui;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelState {
  Disabled,
  Loading,
  //  Paused,
  Loaded,
  LevelComplete,
  BossComplete,
}
#[derive(Component)]
pub struct LevelLoader {
  pub timer: Timer,
}

#[derive(Component)]
pub struct LevelTag;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(TilemapPlugin)
      .add_plugin(player::PlayerPlugin)
      .add_plugin(enemy::EnemyPlugin)
      .add_state(LevelState::Disabled)
      .init_resource::<generator::Level>()
      .init_resource::<ui::Stats>()
      .init_resource::<complete::CompletedLevels>()
      .init_resource::<LevelSettings<WallType, TileType>>()
      .add_system(crate::systems::set_texture_filters_to_nearest)
      // loading
      .add_system_set(
        SystemSet::on_enter(LevelState::Loading)
          //.with_system(camera::setup_camera.label("load").after("cleanup"))
          .with_system(cleanup_system::<camera::MainCamera>)
          .with_system(loading::show_loading)
          .with_system(generate_level.label("load").after("cleanup"))
          .with_system(cleanup_system::<LevelTag>.label("cleanup")),
      )
      .add_system_set(SystemSet::on_update(LevelState::Loading).with_system(load_complete))
      .add_system_set(
        SystemSet::on_exit(LevelState::Loading).with_system(cleanup_system::<loading::LoadingTag>),
      )
      // loaded
      .add_system_set(SystemSet::on_enter(LevelState::Loaded)
        .with_system(camera::setup_camera)
        .with_system(ui::create_ui))
      .add_system_set(
        SystemSet::on_update(LevelState::Loaded)
          .with_system(check_level_complete)
          .with_system(camera::camera_system)
          .with_system(ui::measure_time)
          .with_system(camera::camera_system_initial_focus),
      )
      // level complete
      .add_system_set(
        SystemSet::on_enter(LevelState::LevelComplete)
          .with_system(cleanup_system::<camera::MainCamera>)
          .with_system(complete::show_complete)
          .with_system(ui::count_levels)
          .with_system(generate_level.label("load").after("cleanup"))
          .with_system(cleanup_system::<LevelTag>.label("cleanup")),
      )
      .add_system_set(
        SystemSet::on_update(LevelState::LevelComplete)
          .with_system(complete::wait_to_load_next_level),
      )
      .add_system_set(
        SystemSet::on_exit(LevelState::LevelComplete)
          .with_system(cleanup_system::<complete::CompleteLoadingTag>),
      )
      // disabled
      .add_system_set(
        SystemSet::on_enter(LevelState::Disabled)
          .with_system(cleanup_system::<LevelTag>)
          .with_system(cleanup_system::<camera::MainCamera>)
          .with_system(complete::reset_level_count), //.with_system(despawn_player),
      );
  }
}

impl FromWorld for LevelSettings<WallType, TileType> {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world
      .get_resource::<AssetServer>()
      .expect("should find asset server");
    let texture_handle = asset_server.load("full tilemap.png");

    let mut wall_tiles = HashMap::default();

    wall_tiles.insert(WallType::North, 56);
    wall_tiles.insert(WallType::South, 65);
    wall_tiles.insert(WallType::East, 49);
    wall_tiles.insert(WallType::EastInnerCorner, 58);
    wall_tiles.insert(WallType::West, 48);
    wall_tiles.insert(WallType::WestInnerCorner, 57);
    wall_tiles.insert(WallType::Northeast, 59);
    wall_tiles.insert(WallType::Northwest, 55);
    wall_tiles.insert(WallType::Southeast, 68);
    wall_tiles.insert(WallType::Southwest, 64);

    let mut floor_tiles = HashMap::default();
    floor_tiles.insert(TileType::Dirt, 23);
    floor_tiles.insert(TileType::Exit, 28);

    LevelSettings {
      tilemap: texture_handle,
      tilemap_size: Vec2::new(144., 128.),
      tile_size: Vec2::new(16., 16.),
      chunk_size: (48, 96),
      map_size: (1, 1),
      wall_tiles,
      floor_tiles,
    }
  }
}
