use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_egui::EguiPlugin;
use bevy::prelude::*;


pub mod animation; // TODO: move to own crate
pub mod audio; // TODO: move to own crate
mod splash;
mod menu;
mod game;
mod debug;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum GameState {
    Splash,
    Menu,
    Game,
    //Credits,
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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .add_plugin(audio::AudioPlugin)
        .add_plugin(animation::AnimationPlugin)
        .add_plugin(debug::DebugPlugin)
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

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

