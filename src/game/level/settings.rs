use bevy::{prelude::*, utils::HashMap};
use std::{fmt::Debug, hash::Hash};

#[derive()]
pub struct LevelSettings<TWall, TFloor> {
  pub tilemap: Handle<Image>,
  pub tilemap_size: Vec2,
  pub tile_size: Vec2,
  pub chunk_size: (u32, u32),
  pub map_size: (u32, u32),
  pub wall_tiles: HashMap<TWall, u16>,
  pub floor_tiles: HashMap<TFloor, u16>,
}
impl<TWall, TFloor> LevelSettings<TWall, TFloor>
where
  TWall: Eq + Hash + Debug,
  TFloor: Eq + Hash + Debug,
{
  pub fn get_wall_tile(&self, wall_type: TWall) -> Option<u16> {
    self.wall_tiles.get(&wall_type).map(|&tile_id| tile_id)
  }
  pub fn get_floor_tile(&self, floor_type: TFloor) -> Option<u16> {
    self.floor_tiles.get(&floor_type).map(|&tile_id| tile_id)
  }
}
