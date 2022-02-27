use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{InspectorPlugin, Inspectable};

mod game;
mod menu;
mod splash;
mod systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum GameState {
  Splash,
  Menu,
  Game,
  // Credits,
}

fn main() {
  App::new()
    .insert_resource(WindowDescriptor {
      title: "OP".to_string(),
      width: 1920.,
      height: 1080.,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_state(GameState::Game)
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    //.add_plugin(LogDiagnosticsPlugin::default())
    //.add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(systems::AudioPlugin)
    .add_plugin(systems::AnimationPlugin)
    .add_plugin(systems::CombatPlugin)
    //.add_plugin(systems::DebugPlugin)
    //.add_plugin(InspectorPlugin::<Data>::new())
    .add_plugin(systems::PhysicsPlugin)
    .add_plugin(systems::MousePlugin)
    .add_plugin(systems::MovementPlugin)
    .add_plugin(systems::TopDownCameraPlugin)
    .add_plugin(splash::SplashPlugin)
    .add_plugin(menu::MenuPlugin)
    .add_plugin(game::GamePlugin)
    .add_startup_system(setup)
    .run();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {
  commands.spawn_bundle(UiCameraBundle::default());
}
