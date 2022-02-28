use crate::game::player::PlayerAnimationState;
use crate::systems::AtlasAnimationDefinition;
use crate::systems::SimpleDirection;
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
    },
  );
  animations
}
