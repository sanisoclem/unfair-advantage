use crate::systems::cleanup_system;
use crate::systems::AtlasAnimation;
use crate::systems::AtlasAnimationDefinition;
use bevy::prelude::*;

use super::GameState;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
  fn build(&self, app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state
    // `GameState::Splash`
    app
      // When entering the state, spawn everything needed for this screen
      .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
      // While in this state, run the `countdown` system
      .add_system_set(SystemSet::on_update(GameState::Splash).with_system(countdown))
      // When exiting the state, despawn everything that was spawned for this screen
      .add_system_set(
        SystemSet::on_exit(GameState::Splash).with_system(cleanup_system::<OnSplashScreen>),
      );
  }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
struct SplashTimer(Timer);

fn splash_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let banner = asset_server.load("banner-spritesheet.png");
  let texture_atlas = TextureAtlas::from_grid(banner, Vec2::new(905.0, 363.0), 6, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands
    .spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(OnSplashScreen);

  // Display the logo
  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(1.0)),
      ..Default::default()
    })
    .insert(AtlasAnimationDefinition {
      start: 0,
      end: 5,
      fps: 5.,
      repeat: true,
      random_start: true,
    })
    .insert(AtlasAnimation::default())
    .insert(OnSplashScreen);

  // Insert the timer as a resource
  commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, false)));
}

// Tick the timer, and change state when finished
fn countdown(
  mut game_state: ResMut<State<GameState>>,
  time: Res<Time>,
  mut timer: ResMut<SplashTimer>,
) {
  if timer.0.tick(time.delta()).finished() {
    game_state.set(GameState::Menu).unwrap();
  }
}
