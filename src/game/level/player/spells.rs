use crate::systems::{Spell, SpellSprite, SpellStatus, SpellType};
use bevy::{prelude::*, utils::HashMap};
use heron::prelude::*;

pub fn build_spells(
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> HashMap<SpellType, Spell> {
  let mut spells = HashMap::default();

  let texture_handle = asset_server.load("Dark VFX 8 (72x32).png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(72.0, 32.0), 16, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  spells.insert(
    SpellType::BasicAttack,
    Spell {
      status: SpellStatus::Ready,
      damage_min: 100.,
      damage_max: 1000.,
      dot: false,
      damage_tick: 0.0,
      prepare_duration: 0.3,  // 1.1667/2.,
      cast_duration: 0.05,    // 0.1667/2.,
      recovery_duration: 0.5, // 667/2.,
      prepare_sprite: Some(SpellSprite {
        texture_atlas: texture_atlas_handle.clone(),
        start_frame: 0,
        end_frame: 6,
        repeatable: false,
        fps: 0.,
        translation: Vec2::new(35., 0.),
      }),
      cast_sprite: Some(SpellSprite {
        texture_atlas: texture_atlas_handle.clone(),
        start_frame: 7,
        end_frame: 9,
        repeatable: false,
        fps: 0.,
        translation: Vec2::new(35., 0.),
      }),
      recovery_sprite: Some(SpellSprite {
        texture_atlas: texture_atlas_handle,
        start_frame: 10,
        end_frame: 15,
        repeatable: false,
        fps: 0.,
        translation: Vec2::new(35., 0.),
      }),
      projectile_sprite: None,
      shape: CollisionShape::Cuboid {
        half_extends: Vec3::new(25., 10., 0.),
        border_radius: None,
      },
      projectile_velocity: 0.,
    },
  );

  spells
}
