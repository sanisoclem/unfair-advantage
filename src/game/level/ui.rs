use bevy::prelude::*;

#[derive(Default)]
pub struct Stats {

}

pub fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut stats: ResMut<Stats>) {
  //let font = asset_server.load("Shizuru-Regular.ttf");


  // commands
  //   .spawn_bundle(NodeBundle {
  //     style: Style {
  //       margin: Rect::all(Val::Px(0.0)),
  //       flex_direction: FlexDirection::Row,
  //       align_items: AlignItems::FlexEnd,
  //       ..Default::default()
  //     },
  //     color: Color::CRIMSON.into(),
  //     ..Default::default()
  //   })
  //   .insert(super::LevelTag)
  //   .with_children(|parent| {
  //     parent.spawn_bundle(TextBundle {
  //       style: Style {
  //         margin: Rect::all(Val::Px(50.0)),
  //         ..Default::default()
  //       },
  //       text: Text::with_section(
  //         "Levels Completed: 0",
  //         TextStyle {
  //           font: font.clone(),
  //           font_size: 80.0,
  //           color: Color::rgb(0.9, 0.9, 0.9),
  //         },
  //         Default::default(),
  //       ),
  //       ..Default::default()
  //     });
  //   });
}