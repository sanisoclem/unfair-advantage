use crate::utils::create_hash;
use core::cmp::min;
use pathfinding::prelude::{absdiff, astar};
use rand::prelude::*;

use std::cmp::max;

const SQRT2: f32 = 1.4142135623730950488016887242097;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TileType {
  Nothing,
  Exit,
  Dirt,
}
impl Default for TileType {
  fn default() -> Self {
    TileType::Nothing
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WallType {
  Nothing,
  North,
  South,
  East,
  EastInnerCorner,
  West,
  WestInnerCorner,
  Northeast,
  Northwest,
  Southeast,
  Southwest,
}

impl Default for WallType {
  fn default() -> Self {
    WallType::Nothing
  }
}

// TODO: separate builder/generator
#[derive(Debug, Default, Clone)]
pub struct Level {
  pub width: u32,
  pub height: u32,
  pub tiles: Vec<Vec<LevelTile>>, // quadtrees?
  pub rooms: Vec<Rect>,
  pub collission_shapes: Vec<Rect>,
  pub player_start_position: Point,
  pub exit_point: Point,
  // pub tile_size: Vec2
}

#[derive(Default, Clone, Debug)]
pub struct LevelTile {
  pub position: Point,
  pub tile_type: TileType,
  pub wall_type: WallType,
  pub is_spawn_point: bool,
  pub spawned: bool,
}

impl Level {
  pub fn generate_seed_bytes(seed: [u8; 32], width: u32, height: u32) -> Self {
    let mut retval = Self::new(width, height);
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    retval.place_rooms(&mut rng);
    retval.place_corridors(&mut rng);
    retval.calculate_walls();
    retval.calculate_collission_shapes();
    retval.calculate_spawn_points(&mut rng);
    retval
  }

  // the procedural generation is not very good, so no point in exposing this function
  pub fn generate_seed_str(seed: &str, width: u32, height: u32) -> Self {
    let hash = create_hash(seed);
    let seed_bytes = hash
      .bytes()
      .take(32)
      .collect::<Vec<_>>()
      .try_into()
      .unwrap();

    Self::generate_seed_bytes(seed_bytes, width, height)
  }

  pub fn generate_random(width: u32, height: u32) -> Self {
    let mut seed = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut seed);

    Self::generate_seed_bytes(seed, width, height)
  }

  pub fn new(width: u32, height: u32) -> Self {
    let mut tiles = Vec::new();
    for x in 0..width {
      for y in 0..height {
        let position = Point {
          x: x as i32,
          y: y as i32,
        };
        let tile = LevelTile {
          position,
          ..Default::default()
        };
        tiles.push(tile);
      }
    }
    let tiles = (0..width)
      .map(|_| vec![LevelTile::default(); height as usize])
      .collect();

    Level {
      width,
      height,
      tiles,
      ..Default::default()
    }
  }

  pub fn place_rooms(&mut self, rng: &mut StdRng) {
    // configure room sizes
    let max_rooms = 20;
    let min_room_width = 8;
    let max_room_width = 20;
    let min_room_height = 8;
    let max_room_height = 20;

    for _ in 0..max_rooms {
      // place up to max_rooms - if it collides with another, it won't get placed

      let mut x = rng.gen_range(0..self.width);
      let mut y = rng.gen_range(0..self.height - 1);

      let width = rng.gen_range(min_room_width..=max_room_width);
      let height = rng.gen_range(min_room_height..=max_room_height);

      if x + width > self.width {
        x = self.width - width;
      }

      if y + height > self.height {
        y = self.height - height - 1;
      }

      let mut collides = false;
      let room = Rect::new(x as i32, y as i32, width as i32, height as i32);

      // check all other rooms we've placed to see if this one
      // collides with them
      for other_room in &self.rooms {
        if room.intersects(&other_room) {
          collides = true;
          break;
        }
      }

      // if the new room doesn't collide, add it to the level
      if !collides {
        self.add_room(&room);
      }
    }
  }

  #[inline]
  pub fn get(&self, x: i32, y: i32) -> TileType {
    if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
      TileType::Nothing
    } else {
      self.tiles[x as usize][y as usize].tile_type.clone()
    }
  }

  #[inline]
  pub fn get_wall(&self, x: i32, y: i32) -> WallType {
    if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
      WallType::Nothing
    } else {
      self.tiles[x as usize][y as usize].wall_type.clone()
    }
  }

  #[inline]
  fn set_wall(&mut self, x: i32, y: i32, wall: WallType) {
    self.tiles[x as usize][y as usize].wall_type = wall;
  }

  #[inline]
  fn set(&mut self, x: i32, y: i32, tile: TileType) {
    self.tiles[x as usize][y as usize].tile_type = tile;
  }

  fn add_room(&mut self, room: &Rect) {
    for row in 0..room.height {
      for col in 0..room.width {
        let y = room.y + row;
        let x = room.x + col;

        self.set(x, y, TileType::Dirt);
      }
    }
    self.rooms.push(room.clone());
  }

  pub fn place_corridors(&mut self, rng: &mut StdRng) {
    for i in 0..(self.rooms.len() - 1) {
      let room = self.rooms[i].clone();
      let other = self.rooms[i + 1].clone();

      // randomly pick vert or horz
      match rng.gen_range(0..2) {
        0 => {
          match room.centre.x <= other.centre.x {
            true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
            false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
          }
          match room.centre.y <= other.centre.y {
            true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
            false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x),
          }
        }
        _ => {
          match room.centre.y <= other.centre.y {
            true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
            false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x),
          }
          match room.centre.x <= other.centre.x {
            true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
            false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
          }
        }
      }
    }
  }

  fn calculate_walls(&mut self) {
    for x in 0..(self.width as i32) {
      for y in 0..(self.height as i32) {
        if self.get(x, y) == TileType::Nothing {
          if self.get(x, y - 1) == TileType::Dirt {
            if self.get(x - 1, y - 1) == TileType::Nothing {
              self.set_wall(x, y, WallType::Northwest);
            } else if self.get(x + 1, y - 1) == TileType::Nothing {
              self.set_wall(x, y, WallType::Northeast);
            } else {
              self.set_wall(x, y, WallType::North);
            }
          }
        } else {
          if self.get(x - 1, y) == TileType::Nothing {
            if self.get(x, y - 1) == TileType::Nothing {
              self.set_wall(x, y, WallType::Southwest);
            } else if self.get(x - 1, y - 1) != TileType::Nothing {
              self.set_wall(x, y, WallType::WestInnerCorner);
            } else {
              self.set_wall(x, y, WallType::West);
            }
          } else if self.get(x + 1, y) == TileType::Nothing {
            if self.get(x, y - 1) == TileType::Nothing {
              self.set_wall(x, y, WallType::Southeast);
            } else if self.get(x + 1, y - 1) != TileType::Nothing {
              self.set_wall(x, y, WallType::EastInnerCorner);
            } else {
              self.set_wall(x, y, WallType::East);
            }
          } else if self.get(x, y - 1) == TileType::Nothing {
            self.set_wall(x, y, WallType::South);
          }
        }
      }
    }
  }

  fn horz_corridor(&mut self, start_x: i32, end_x: i32, y: i32) {
    for y1 in y - 1..y + 2 {
      for col in start_x..end_x + 1 {
        self.set(col, y1, TileType::Dirt);
      }
    }
  }

  fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
    for x1 in x - 1..x + 2 {
      for row in start_y..end_y + 1 {
        self.set(x1, row, TileType::Dirt);
      }
    }
  }

  pub fn get_tiles(&self) -> impl Iterator<Item = (i32, i32, &LevelTile)> {
    self.tiles.iter().enumerate().flat_map(|(x, ys)| {
      ys.iter()
        .enumerate()
        .map(move |(y, t)| (x as i32, y as i32, t))
    })
  }

  pub fn get_tiles_mut(&mut self) -> impl Iterator<Item = (i32, i32, &mut LevelTile)> {
    self.tiles.iter_mut().enumerate().flat_map(|(x, ys)| {
      ys.iter_mut()
        .enumerate()
        .map(move |(y, t)| (x as i32, y as i32, t))
    })
  }

  pub fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut LevelTile {
    &mut self.tiles[x as usize][y as usize]
  }

  pub fn get_tile(&self, x: i32, y: i32) -> &LevelTile {
    &self.tiles[x as usize][y as usize]
  }

  fn calculate_collission_shapes(&mut self) {
    let mut rects = self
      .get_tiles()
      .map(|(x, y, t)| (x, y, t.tile_type.clone(), t.wall_type.clone()))
      .flat_map(|(x, y, t, w)| match (t, w) {
        (TileType::Nothing, _) => vec![Some(Rect::new(x * 3, y * 3, 3, 3))],
        (_, WallType::West) | (_, WallType::WestInnerCorner) => {
          vec![Some(Rect::new(x * 3, y * 3, 1, 3))]
        }
        (_, WallType::East) | (_, WallType::EastInnerCorner) => {
          vec![Some(Rect::new(x * 3 + 2, y * 3, 1, 3))]
        }
        (_, WallType::South) => vec![Some(Rect::new(x * 3, y * 3, 3, 1))],
        (_, WallType::Southeast) => vec![
          Some(Rect::new(x * 3, y * 3, 3, 1)),
          Some(Rect::new(x * 3 + 2, y * 3 + 1, 1, 2)),
        ],
        (_, WallType::Southwest) => vec![
          Some(Rect::new(x * 3, y * 3, 3, 1)),
          Some(Rect::new(x * 3, y * 3 + 1, 1, 2)),
        ],
        _ => vec![None],
      })
      .filter_map(|i| i)
      .collect::<Vec<_>>();

    for _ in 0..10 {
      merge_rects(&mut rects);
    }
    self.collission_shapes = rects;
  }

  fn calculate_spawn_points(&mut self, rng: &mut StdRng) {
    let mut spawn_points: Vec<Point> = Vec::new();
    let min_dist = 1;
    let num_rooms = self.rooms.len();
    self.rooms.sort_by_key(|r| r.centre.y);
    self.exit_point = self.rooms[num_rooms - 1].centre;
    self.player_start_position = self.rooms[0].centre;

    for room in self.rooms.iter_mut().skip(1).take(num_rooms - 2) {
      for _i in 0..room.area() {
        let x = rng.gen_range(room.x + 1..room.x2 - 2);
        let y = rng.gen_range(room.y + 1..room.y2 - 2);
        let p = Point { x, y };

        spawn_points.retain(|p2| (p2.x - p.x).abs() + (p2.y - p.y).abs() > min_dist);
        spawn_points.push(p);
      }
    }

    for p in spawn_points {
      self.tiles[p.x as usize][p.y as usize].is_spawn_point = true;
    }
    self.set(self.exit_point.x, self.exit_point.y, TileType::Exit);
  }

  fn get_neighbors(&self, pos: &Point) -> impl Iterator<Item = (Point, i32)> {
    let x = pos.x;
    let y = pos.y;

    let mut neighbors = Vec::new();
    let mut s0 = false;
    let mut s1 = false;
    let mut s2 = false;
    let mut s3 = false;
    let d0;
    let d1;
    let d2;
    let d3;

    // ↑
    if self.get(x, y - 1) != TileType::Nothing {
      neighbors.push(Point { x, y: y - 1 });
      s0 = true;
    }
    // →
    if self.get(x + 1, y) != TileType::Nothing {
      neighbors.push(Point { x: x + 1, y });
      s1 = true;
    }
    // ↓
    if self.get(x, y + 1) != TileType::Nothing {
      neighbors.push(Point { x, y: y + 1 });
      s2 = true;
    }
    // ←
    if self.get(x - 1, y) != TileType::Nothing {
      neighbors.push(Point { x: x - 1, y });
      s3 = true;
    }

    d0 = s3 && s0;
    d1 = s0 && s1;
    d2 = s1 && s2;
    d3 = s2 && s3;

    // ↖
    if d0 && self.get(x - 1, y - 1) != TileType::Nothing {
      neighbors.push(Point { x: x - 1, y: y - 1 });
    }
    // ↗
    if d1 && self.get(x + 1, y - 1) != TileType::Nothing {
      neighbors.push(Point { x: x + 1, y: y - 1 });
    }
    // ↘
    if d2 && self.get(x + 1, y + 1) != TileType::Nothing {
      neighbors.push(Point { x: x + 1, y: y + 1 });
    }
    // ↙
    if d3 && self.get(x - 1, y + 1) != TileType::Nothing {
      neighbors.push(Point { x: x - 1, y: y + 1 });
    }

    return neighbors.into_iter().map(|p| (p, 1));
  }

  pub fn get_path(&self, from: Point, to: Point) -> Option<Vec<Point>> {
    println!("Finding path from {:?} to {:?}", from, to);
    dbg!(astar(
      &from,
      |p| self.get_neighbors(p),
      |p| p.distance(&to) as i32 / 3,
      |p| *p == to,
    ).map(|(mut v, _)| {v.remove(0); v } ))
  }

  // fn get_valid_destination(&self, from: Point, candidate: Point) -> Point {

  // }
}

fn merge_rects(rects: &mut Vec<Rect>) {
  for i in 0..rects.len() {
    if rects[i].merged {
      continue;
    }
    for j in i + 1..rects.len() {
      if rects[j].merged {
        continue;
      }
      if rects[i].can_merge(&rects[j]) {
        rects[j].merged = true;
        let other = rects[j].clone();
        rects[i].merge(&other);
        break;
      }
    }
  }
  rects.retain(|r| !r.merged);
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  fn distance(&self, other: &Self) -> u32 {
    (absdiff(self.x, other.x) + absdiff(self.y, other.y)) as u32
  }
}

#[derive(Clone, Debug)]
pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub x2: i32,
  pub y2: i32,
  pub width: i32,
  pub height: i32,
  pub centre: Point,
  pub merged: bool,
}

impl Rect {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
    Rect {
      x,
      y,
      x2: x + width,
      y2: y + height,
      width,
      height,
      centre: Point {
        x: x + (width / 2),
        y: y + (height / 2),
      },
      merged: false,
    }
  }

  pub fn intersects(&self, other: &Self) -> bool {
    self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
  }

  pub fn can_merge(&self, other: &Self) -> bool {
    (self.x == other.x && self.x2 == other.x2 && (self.y == other.y2 || self.y2 == other.y))
      || (self.y == other.y && self.y2 == other.y2 && (self.x == other.x2 || self.x2 == other.x))
  }
  pub fn merge(&mut self, other: &Self) {
    if self.x == other.x && self.x2 == other.x2 {
      self.y = min(self.y, other.y);
      self.y2 = max(self.y2, other.y2);
    } else if self.y == other.y && self.y2 == other.y2 {
      self.x = min(self.x, other.x);
      self.x2 = max(self.x2, other.x2);
    } else {
      panic!("cannot merge rects");
    }
    self.width = self.x2 - self.x;
    self.height = self.y2 - self.y;
    self.centre = Point {
      x: self.x + (self.width / 2),
      y: self.y + (self.height / 2),
    };
  }
  #[inline]
  pub fn area(&self) -> i32 {
    self.width * self.height
  }
}
