use bevy::prelude::*;

use super::{despawn_screen, GameState};

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
        SystemSet::on_exit(GameState::Splash).with_system(despawn_screen::<OnSplashScreen>),
      );
  }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let icon = asset_server.load("banner.png");
  // Display the logo
  commands
    .spawn_bundle(ImageBundle {
      style: Style {
        // This will center the logo
        margin: Rect::all(Val::Auto),
        // This will set the logo to be 200px wide, and auto adjust its height
        size: Size::new(Val::Px(900.0), Val::Auto),
        ..Default::default()
      },
      image: UiImage(icon),
      ..Default::default()
    })
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
