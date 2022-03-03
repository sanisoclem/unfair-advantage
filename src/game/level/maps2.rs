use bevy::prelude::*;
use core::cmp::min;
use heron::prelude::*;
use rand::prelude::*;
use sha2::{Digest, Sha256};
use std::cmp::max;

fn create_hash(text: &str) -> String {
  let mut hasher = Sha256::default();
  hasher.update(text.as_bytes());
  format!("{:x}", hasher.finalize())
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
  Nothing,
  Floor,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, Default)]
pub struct Level {
  pub width: u32,
  pub height: u32,
  pub board: Vec<Vec<TileType>>,
  pub walls: Vec<Vec<WallType>>,
  pub rooms: Vec<Rect>,
  pub collission_shapes: Vec<Rect>,
  pub spawn_points: Vec<Point>,
  pub player_start_position: Point,
  pub exit_point: Point,
}

impl Level {
  pub fn generate(seed: &str, width: u32, height: u32) -> Self {
    let mut retval = Self::new(width, height);
    let hash = create_hash(seed);

    let mut rng: StdRng = SeedableRng::from_seed(
      hash
        .bytes()
        .take(32)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap(),
    );
    retval.place_rooms(&mut rng);
    retval.place_corridors(&mut rng);
    retval
  }
  pub fn generate_random(width: u32, height: u32) -> Self {
    let mut retval = Self::new(width, height);

    let mut seed = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut seed);

    let mut rng: StdRng = SeedableRng::from_seed(seed);
    retval.place_rooms(&mut rng);
    retval.place_corridors(&mut rng);
    retval.calculate_walls();
    retval.calculate_collission_shapes();
    retval.calculate_spawn_points();
    retval
  }
  pub fn new(width: u32, height: u32) -> Self {
    let board = (0..width)
      .map(|_| vec![TileType::Nothing; height as usize])
      .collect();
    let walls = (0..width)
      .map(|_| vec![WallType::Nothing; height as usize])
      .collect();

    Level {
      width,
      height,
      walls,
      board,
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

      // if it's off the board, shift it back on again
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

  pub fn get(&self, x: i32, y: i32) -> TileType {
    if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
      TileType::Nothing
    } else {
      self.board[x as usize][y as usize].clone()
    }
  }
  pub fn get_wall(&self, x: i32, y: i32) -> WallType {
    if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
      WallType::Nothing
    } else {
      self.walls[x as usize][y as usize].clone()
    }
  }
  #[inline]
  fn set_wall(&mut self, x: i32, y: i32, wall: WallType) {
    self.walls[x as usize][y as usize] = wall;
  }

  #[inline]
  fn set(&mut self, x: i32, y: i32, tile: TileType) {
    self.board[x as usize][y as usize] = tile;
  }

  fn add_room(&mut self, room: &Rect) {
    // loop through all items in the board which are covered by the new room
    // and change them to '1' which indicates they are not empty
    for row in 0..room.height {
      for col in 0..room.width {
        let y = room.y + row;
        let x = room.x + col;

        self.set(x, y, TileType::Floor);
        //info!("set {:?}, {:?} to {:?}", x, y, TileType::Floor);
      }
    }

    // also keep track of rooms so that we can check for collisions
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
          if self.get(x, y - 1) == TileType::Floor {
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
        self.set(col, y1, TileType::Floor);
      }
    }
  }

  fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
    for x1 in x - 1..x + 2 {
      for row in start_y..end_y + 1 {
        self.set(x1, row, TileType::Floor);
      }
    }
  }

  fn calculate_collission_shapes(&mut self) {
    let mut rects = self
      .board
      .iter()
      .enumerate()
      .flat_map(|(x, ys)| ys.iter().enumerate().map(move |(y, t)| (x, y, t.clone())))
      .zip(
        self
          .walls
          .iter()
          .flat_map(|ys| ys.iter().map(move |t| t.clone())),
      )
      .flat_map(|((x, y, t), w)| match (t, w) {
        (TileType::Nothing, _) => vec![Some(Rect::new(x as i32 * 3, y as i32 *3, 3, 3))],
        (_, WallType::West) | (_, WallType::WestInnerCorner) => vec![Some(Rect::new(x as i32 * 3, y as i32 *3, 1, 3))],
        (_, WallType::East) | (_, WallType::EastInnerCorner) => vec![Some(Rect::new(x as i32 * 3 + 2, y as i32 *3, 1, 3))],
        (_, WallType::South) => vec![Some(Rect::new(x as i32 * 3 , y as i32 *3, 3, 1))],
        (_, WallType::Southeast) => vec![Some(Rect::new(x as i32 * 3 , y as i32 *3, 3, 1)), Some(Rect::new(x as i32 * 3 + 2, y as i32 * 3 + 1, 1, 2))],
        (_, WallType::Southwest) => vec![Some(Rect::new(x as i32 * 3 , y as i32 *3, 3, 1)), Some(Rect::new(x as i32 * 3, y as i32 * 3 + 1, 1, 2))],
        _ => vec![None],
      })
      .filter_map(|i|i)
      .collect::<Vec<_>>();

    for _ in 0..10 {
      merge_rects(&mut rects);
    }
    self.collission_shapes = rects;
  }

  fn calculate_spawn_points(&mut self) {
    self.rooms.sort_by_key(|r| r.centre.y);
    self.exit_point = self.rooms[self.rooms.len() -1].centre;
    self.player_start_position = self.rooms[0].centre;
  }
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
  pub x: i32,
  pub y: i32,
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
  pub fn contains_tile(&self, x: i32, y: i32) -> bool {
    x >= self.x && x < self.x2 && y >= self.y && y < self.y2
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
}
