use bevy::prelude::*;
use std::marker::PhantomData;

pub enum TileType {

}

pub struct Map {
  pub tiles: Vec<TileType>,
  pub width: usize,
  pub height: usize,
}

pub fn generate_map(width: usize, height: usize) -> Map {
  let rect = Rect { left: 0, top: 0, right: width, bottom: height };

  unimplemented!()
}

pub fn slice_random_rect(rects: &Vec<Rect<usize>>, min_height: usize, min_width: usize) {

}



// pub struct QuadTree<T>{
//   phatom: PhantomData<T>
// }

// pub fn generate_level() -> QuadTree<TileType> {
//   unimplemented!()
// }

// pub fn generate_points_in_circle(radius: f32, num_points: usize) -> Vec<Vec2> {
//   let mut points = Vec::new();
//   for _ in 0..num_points {
//     let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
//     let length = rand::random::<f32>();
//     let x = (radius * angle.cos()).round();
//     let y = (radius * angle.sin()).round();
//     points.push(Vec2::new(x, y) * length);
//   }
//   points
// }


// pub fn generate_rooms() -> Vec<(Vec2, Vec2)> {
//   let max_height = 1.0;
//   let max_width = 1.0;
//   let points = generate_points_in_circle(1.0, 50);

//   let mut rects : Vec<_> = points.into_iter().map(|point| {
//     let height = (rand::random::<f32>() * max_height/2.);//.sqrt();
//     let width = (rand::random::<f32>() * max_width/2.);//.sqrt();
//     (point, Vec2::new(width, height))
//   }).collect();

//   let rects2 = rects.clone();

//   // for (s1, s2) in rects.iter_mut() {
//   //   let mut separation = Vec2::new(0., 0.);
//   //   for (o1, o2) in rects2.iter() {
//   //     if s1 == o1 && s2 == o2 {
//   //       continue;
//   //     }
//   //     separation += *o1 - *s1;
//   //   }
//   //   *s1 += separation;
//   // }

//   rects
// }

// // pub fn separate_rooms(rooms: Vec<(Vec2, Vec2)>) -> Vec<(Vec2, Vec2)> {
// //   let count = rooms.len();

// // }

// pub fn generate_rooms2(iterations: usize, num_rooms: usize) -> Vec<(Vec2, Vec2)> {
//   let mut xs = Vec::new();
//   let mut ys = Vec::new();
//   for _ in 0..iterations {
//     ys.push(rand::random::<f32>());
//     let nx = rand::random::<f32>();
//     for x in xs.iter_mut() {
//       if *x < nx {
//         *x = 1. - *x;
//       }
//     }
//     xs.push(nx);
//   }


//   xs.push(1.);
//   ys.push(1.);

//   xs.sort_by(|a, b| a.partial_cmp(b).expect("sort f32s failed"));
//   ys.sort_by(|a, b| a.partial_cmp(b).expect("sort f32s failed"));

//   println!("xs {:?}", xs);
//   println!("ys {:?}", ys);

//   let mut prev_x = 0.0;

//   let mut retval = Vec::new();

//   for x in xs.iter() {

//   let mut prev_y = 0.0;
//     for y in ys.iter() {
//       retval.push((Vec2::new(prev_x, prev_y), Vec2::new(*x, *y)));
//       prev_y = *y;
//     }

//     prev_x = *x;
//   }

//   println!("retval {:?}", retval);

//   retval.sort_by(|(a1, a2),(b1,b2)| {
//     let area1 = *a2 - *a1;
//     let area2 = *b2 - *b1;
//     area1.length().partial_cmp(&area2.length()).expect("sort f32s failed")
//   });

//   //retval.into_iter().take(num_rooms).collect()
//   retval.into_iter().collect()
// }

// pub struct Room {
//   pos: Vec2,
//   size: Vec2,
//   room_type: RoomType
// }

// #[derive(Clone)]
// pub enum RoomType {
//   Tiny,
//   Small,
//   Medium,
//   Large,
//   Boss
// }

// pub struct Ruleset<T1,T2> {
//   rules: Vec<fn (&T1, &[T2], usize, usize) -> bool>
// }
// impl<T1,T2> Ruleset<T1,T2> where T1: Clone {
//   pub fn run(&self, all_items: &[T1], prev: &[T2], num: usize, level_num: usize) -> Vec<T1> {
//     let mut retval = Vec::new();
//     for rule in self.rules.iter() {
//       for item in all_items.iter() {
//         if rule(item, prev, num, level_num) {
//           retval.push(item.clone());
//         }
//       }
//     }
//     retval
//   }
// }

// pub fn generate_rooms3(num: usize, level_num: usize) -> Vec<Room> {
//   let room_types = generate_room_types(num, level_num);
//   unimplemented!()
// }

// // pub fn generate_exits(num: usize) ->Vec<Vec2> {
// //   let mut prev = None;
// //   for x in 0..num {
// //     match prev {
// //       None => rand::random::<f32>() * 2. * std::f32::consts::PI,
// //       Some(prev) =>
// //     }
// //   }
// // }

// pub fn generate_room_types(num: usize, level_num: usize) -> Vec<RoomType> {
//   let ruleset = Ruleset {
//     rules: vec![room_type_rules::should_start_in_tiny_room]
//   };

//   let room_types = vec![
//     RoomType::Tiny,
//     RoomType::Small,
//     RoomType::Medium,
//     RoomType::Large,
//     RoomType::Boss
//   ];
//   let mut retval = Vec::new();
//   for _ in 0..num {
//     let valid_types = ruleset.run(&room_types, &retval, num, level_num);
//     if valid_types.len() == 0 {
//       panic!("no valid room types");
//     }

//     let room_type = valid_types.get(rand::random::<usize>() % valid_types.len()).expect("should always be inside bounds").clone();
//     retval.push(room_type);
//   }
//   retval
// }

// mod room_type_rules {
//   use super::*;
//   pub fn should_start_in_tiny_room(room_type: &RoomType, prev: &[RoomType], _num: usize, _level_num: usize) -> bool {
//     match (room_type, prev.len()) {
//       (RoomType::Tiny, 0) => true,
//       (_, 0) => false,
//       _ => true
//     }
//   }
// }