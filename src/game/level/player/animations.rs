use super::PlayerAnimationState;
use crate::systems::{AtlasAnimationDefinition, SimpleDirection};
use bevy::utils::HashMap;

pub fn build_animations(
) -> HashMap<(SimpleDirection, PlayerAnimationState), AtlasAnimationDefinition> {
  let mut animations = HashMap::default();
  animations.insert(
    (SimpleDirection::East, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216,
      end: 227,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::North, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216 + 24,
      end: 227 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::NorthWest, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24,
      end: 227 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::NorthEast, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::South, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::SouthEast, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::SouthWest, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::West, PlayerAnimationState::Idle),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );

  animations.insert(
    (SimpleDirection::East, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216,
      end: 227,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::North, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216 + 24,
      end: 227 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::NorthWest, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24,
      end: 227 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::NorthEast, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::South, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::SouthEast, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::SouthWest, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::West, PlayerAnimationState::Running),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );

  // DASH
  animations.insert(
    (SimpleDirection::East, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216,
      end: 227,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::North, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216 + 24,
      end: 227 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::NorthWest, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24,
      end: 227 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::NorthEast, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::South, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::SouthEast, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::SouthWest, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::West, PlayerAnimationState::Dashing),
    AtlasAnimationDefinition {
      start: 216 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 227 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: true,
      random_start: true,
      repeat_from: None,
    },
  );

  // PREPARE
  animations.insert(
    (SimpleDirection::East, PlayerAnimationState::PreparingSpell),
    AtlasAnimationDefinition {
      start: 24,
      end: 37,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::North, PlayerAnimationState::PreparingSpell),
    AtlasAnimationDefinition {
      start: 24 + 24,
      end: 37 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::NorthEast,
      PlayerAnimationState::PreparingSpell,
    ),
    AtlasAnimationDefinition {
      start: 24 + 24 + 24,
      end: 37 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::NorthWest,
      PlayerAnimationState::PreparingSpell,
    ),
    AtlasAnimationDefinition {
      start: 24 + 24 + 24 + 24,
      end: 37 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::South, PlayerAnimationState::PreparingSpell),
    AtlasAnimationDefinition {
      start: 24 + 24 + 24 + 24 + 24,
      end: 37 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::SouthEast,
      PlayerAnimationState::PreparingSpell,
    ),
    AtlasAnimationDefinition {
      start: 24 + 24 + 24 + 24 + 24 + 24,
      end: 37 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::SouthWest,
      PlayerAnimationState::PreparingSpell,
    ),
    AtlasAnimationDefinition {
      start: 24 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 37 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::West, PlayerAnimationState::PreparingSpell),
    AtlasAnimationDefinition {
      start: 24 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 37 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );

  // CAST
  animations.insert(
    (SimpleDirection::East, PlayerAnimationState::CastingSpell),
    AtlasAnimationDefinition {
      start: 38,
      end: 39,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::North, PlayerAnimationState::CastingSpell),
    AtlasAnimationDefinition {
      start: 38 + 24,
      end: 39 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::NorthEast,
      PlayerAnimationState::CastingSpell,
    ),
    AtlasAnimationDefinition {
      start: 38 + 24 + 24,
      end: 39 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::NorthWest,
      PlayerAnimationState::CastingSpell,
    ),
    AtlasAnimationDefinition {
      start: 38 + 24 + 24 + 24,
      end: 39 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::South, PlayerAnimationState::CastingSpell),
    AtlasAnimationDefinition {
      start: 38 + 24 + 24 + 24 + 24,
      end: 39 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::SouthEast,
      PlayerAnimationState::CastingSpell,
    ),
    AtlasAnimationDefinition {
      start: 38 + 24 + 24 + 24 + 24 + 24,
      end: 39 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::SouthWest,
      PlayerAnimationState::CastingSpell,
    ),
    AtlasAnimationDefinition {
      start: 38 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 39 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (SimpleDirection::West, PlayerAnimationState::CastingSpell),
    AtlasAnimationDefinition {
      start: 38 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 39 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );

  // Recovering
  animations.insert(
    (
      SimpleDirection::East,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40,
      end: 47,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::North,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40 + 24,
      end: 47 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::NorthEast,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40 + 24 + 24,
      end: 47 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::NorthWest,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40 + 24 + 24 + 24,
      end: 47 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::South,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40 + 24 + 24 + 24 + 24,
      end: 47 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::SouthEast,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40 + 24 + 24 + 24 + 24 + 24,
      end: 47 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::SouthWest,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 47 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations.insert(
    (
      SimpleDirection::West,
      PlayerAnimationState::RecoveringFromSpell,
    ),
    AtlasAnimationDefinition {
      start: 40 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      end: 47 + 24 + 24 + 24 + 24 + 24 + 24 + 24,
      fps: 10.,
      repeat: false,
      random_start: false,
      repeat_from: None,
    },
  );
  animations
}
