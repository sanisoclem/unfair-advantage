use crate::systems::AtlasAnimationDefinition;
use bevy::prelude::*;
use bevy::utils::HashMap;
use core::marker::PhantomData;
use std::hash::Hash;

#[derive(Component, Default)]
pub struct TopDownCharacter<T> {
  pub direction_vec: Vec2,
  pub direction: SimpleDirection,
  pub state: T,
  pub animations: HashMap<(SimpleDirection, T), AtlasAnimationDefinition>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum SimpleDirection {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

impl Default for SimpleDirection {
  fn default() -> Self {
    SimpleDirection::South
  }
}
impl From<Vec2> for SimpleDirection {
  fn from(v: Vec2) -> Self {
    let x = v.x;
    let y = v.y;

    if (x.abs() - y.abs()).abs() < 0.35 {
      if x >= 0.0 && y >= 0. {
        SimpleDirection::NorthEast
      } else if x >= 0. && y < 0. {
        SimpleDirection::SouthEast
      } else if x < 0. && y >= 0.0 {
        SimpleDirection::NorthWest
      } else {
        SimpleDirection::SouthWest
      }
    } else {
      if x.abs() > y.abs() {
        if x >= 0.0 {
          SimpleDirection::East
        } else {
          SimpleDirection::West
        }
      } else {
        if y >= 0.0 {
          SimpleDirection::North
        } else {
          SimpleDirection::South
        }
      }
    }
  }
}

pub struct CharacterPlugin<T> {
  phantom: PhantomData<T>,
}
impl<T> Default for CharacterPlugin<T> {
  fn default() -> Self {
    Self {
      phantom: PhantomData,
    }
  }
}

impl<T> Plugin for CharacterPlugin<T>
where
  T: Component + Hash + Eq + PartialEq + Copy,
{
  fn build(&self, app: &mut App) {
    app
      .add_system(Self::create_animations)
      .add_system(Self::update_animations);
  }
}

impl<T> CharacterPlugin<T>
where
  T: Component + Hash + Eq + PartialEq + Copy,
{
  fn update_animations(
    mut qry: Query<
      (&mut TopDownCharacter<T>, &mut AtlasAnimationDefinition),
      Changed<TopDownCharacter<T>>,
    >,
  ) {
    for (mut character, mut animation) in qry.iter_mut() {
      character.direction = SimpleDirection::from(character.direction_vec);
      if let Some(anim) = character
        .animations
        .get(&(character.direction, character.state))
      {
        *animation = anim.clone();

      info!("animation updated {:?}", character.direction);
      }
    }
  }

  fn create_animations(
    mut commands: Commands,
    qry: Query<(Entity, &TopDownCharacter<T>), Without<AtlasAnimationDefinition>>,
  ) {
    for (entity, character) in qry.iter() {
      let anim = character
        .animations
        .get(&(character.direction, character.state))
        .expect("No animation for direction");
      commands.entity(entity).insert(anim.clone());
      info!("inserted new animation")
    }
  }
}
