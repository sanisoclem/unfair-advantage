use bevy::prelude::*;
use rand::prelude::*;
use sha2::{Digest, Sha256};

fn create_hash(text: &str) -> String {
  let mut hasher = Sha256::default();
  hasher.update(text.as_bytes());
  format!("{:x}", hasher.finalize())
}

#[derive(Clone, Debug)]
pub enum TileType {
  Nothing,
  Wall,
  Floor,
}

#[derive(Debug)]
pub struct Level {
  pub width: u32,
  pub height: u32,
  pub board: Vec<TileType>,
  pub rooms: Vec<Room>,
}

impl Level {
  pub fn generate(seed: &str, width: u32, height: u32) -> Self {
    info!("Generating level with seed: {}", seed);
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

    let mut seed = [0u8;32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut seed);

    let mut rng: StdRng = SeedableRng::from_seed(seed);
    retval.place_rooms(&mut rng);
    retval.place_corridors(&mut rng);
    retval
  }
  pub fn new(width: u32, height: u32) -> Self {
    let board = vec![TileType::Nothing; (width * height) as usize];

    Level {
      width,
      height,
      board,
      rooms: Vec::new(),
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
      let mut y = rng.gen_range(0..self.height);

      let width = rng.gen_range(min_room_width..=max_room_width);
      let height = rng.gen_range(min_room_height..=max_room_height);

      // if it's off the board, shift it back on again
      if x + width > self.width {
        x = self.width - width;
      }

      if y + height > self.height {
        y = self.height - height;
      }

      let mut collides = false;
      let room = Room::new(x as i32, y as i32, width as i32, height as i32);

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
        info!("Room added {:?}", room);
        self.add_room(&room);
      }
    }
  }

  pub fn get(&self, x: u32, y: u32) -> TileType {
    self.board[(y * self.width + x) as usize].clone()
  }

  fn add_room(&mut self, room: &Room) {
    // loop through all items in the board which are covered by the new room
    // and change them to '1' which indicates they are not empty
    for row in 0..room.height {
      for col in 0..room.width {
        let y = (room.y + row) as usize;
        let x = (room.x + col) as usize;

        self.board[y * self.width as usize + x] = TileType::Floor;
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

  fn horz_corridor(&mut self, start_x: i32, end_x: i32, y: i32) {
    for y1 in  y-1..y+2 {
      for col in start_x..end_x + 1 {
        self.board[(y1 * self.width as i32 + col) as usize] = TileType::Floor;
      }
    }
  }

  fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
    for x1 in x-1..x+2 {
      for row in start_y..end_y + 1 {
        self.board[(row * self.width as i32 + x1) as usize] = TileType::Floor;
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Room {
  pub x: i32,
  pub y: i32,
  pub x2: i32,
  pub y2: i32,
  pub width: i32,
  pub height: i32,
  pub centre: Point,
}

impl Room {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
    Room {
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
    }
  }

  pub fn intersects(&self, other: &Self) -> bool {
    self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
  }
}
