use bevy::prelude::*;

#[derive(Component)]
pub struct SoundEffectSource; // TODO

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(play_temp_music)
      .add_system(play_effects);
  }
}

fn play_temp_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
  //let music = asset_server.load("sounds/Windless Slopes.ogg");
  //audio.play(music);
}

fn play_effects(_qry: Query<&SoundEffectSource>) {
  // TODO
}


