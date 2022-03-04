use bevy::prelude::*;

#[derive(Default)]
pub struct Stats {
  pub kills: u32,
  pub time_elapsed: f32,
  pub levels_completed: u32,
}

#[derive(Component)]
pub struct StatsComponent;

pub fn measure_time(
  time: Res<Time>,
  mut stats: ResMut<Stats>,
  mut qry: Query<&mut Text, With<StatsComponent>>,
) {
  stats.time_elapsed += time.delta_seconds();

  for mut c in qry.iter_mut() {
    if stats.levels_completed > 0 {
      c.sections[0].value = format!(
        " {} levels {} kills {:.0} kills/s",
        stats.levels_completed,
        stats.kills,
        stats.kills as f32 / stats.time_elapsed,

      );
    } else {
      c.sections[0].value = format!(
        "{} kills {:.0} kills/s",
        stats.kills,
        stats.kills as f32 / stats.time_elapsed
      );
    }
  }
}

pub fn count_levels(mut stats: ResMut<Stats>) {
  stats.levels_completed += 1;
}

pub fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("Shizuru-Regular.ttf");

  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        margin: Rect::all(Val::Px(0.0)),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        ..Default::default()
      },
      color: Color::NONE.into(),
      ..Default::default()
    })
    .insert(super::LevelTag)
    .with_children(|parent| {
      parent
        .spawn_bundle(NodeBundle {
          style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
          },
          color: Color::CRIMSON.into(),
          ..Default::default()
        })
        .with_children(|parent| {
          parent
            .spawn_bundle(TextBundle {
              style: Style {
                margin: Rect::all(Val::Px(10.0)),
                ..Default::default()
              },
              text: Text::with_section(
                "",
                TextStyle {
                  font: font.clone(),
                  font_size: 80.0,
                  color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
              ),
              ..Default::default()
            })
            .insert(StatsComponent);
        });
    });
}
