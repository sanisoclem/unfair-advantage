use bevy::prelude::*;

#[derive(Component)]
pub struct LoadingTag;

pub fn show_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("Shizuru-Regular.ttf");

  commands
  .spawn_bundle(TextBundle {
    style: Style {
      margin: Rect::all(Val::Px(50.0)),
      ..Default::default()
    },
    text: Text::with_section(
      "Loading...",
      TextStyle {
        font: font.clone(),
        font_size: 80.0,
        color: Color::CRIMSON.into(),
      },
      Default::default(),
    ),
    ..Default::default()
  })
  .insert(LoadingTag);
}

// fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
//   let font = asset_server.load("Shizuru-Regular.ttf");
//   // Common style for all buttons on the screen
//   let button_style = Style {
//     size: Size::new(Val::Px(250.0), Val::Px(65.0)),
//     margin: Rect::all(Val::Px(20.0)),
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     ..Default::default()
//   };
//   let button_icon_style = Style {
//     size: Size::new(Val::Px(30.0), Val::Auto),
//     // This takes the icons out of the flexbox flow, to be positionned exactly
//     position_type: PositionType::Absolute,
//     // The icon will be close to the left border of the button
//     position: Rect {
//       left: Val::Px(10.0),
//       right: Val::Auto,
//       top: Val::Auto,
//       bottom: Val::Auto,
//     },
//     ..Default::default()
//   };
//   let button_text_style = TextStyle {
//     font: font.clone(),
//     font_size: 40.0,
//     color: TEXT_COLOR,
//   };

//   commands
//     .spawn_bundle(NodeBundle {
//       style: Style {
//         margin: Rect::all(Val::Auto),
//         flex_direction: FlexDirection::ColumnReverse,
//         align_items: AlignItems::Center,
//         ..Default::default()
//       },
//       color: Color::CRIMSON.into(),
//       ..Default::default()
//     })
//     .insert(OnMainMenuScreen)
//     .with_children(|parent| {
//       // Display the game name
//       parent.spawn_bundle(TextBundle {
//         style: Style {
//           margin: Rect::all(Val::Px(50.0)),
//           ..Default::default()
//         },
//         text: Text::with_section(
//           "Main Menu UI",
//           TextStyle {
//             font: font.clone(),
//             font_size: 80.0,
//             color: TEXT_COLOR,
//           },
//           Default::default(),
//         ),
//         ..Default::default()
//       });

//       parent
//         .spawn_bundle(ButtonBundle {
//           style: button_style.clone(),
//           color: NORMAL_BUTTON.into(),
//           ..Default::default()
//         })
//         .insert(MenuButtonAction::Play)
//         .with_children(|parent| {
//           let icon = asset_server.load("right.png");
//           parent.spawn_bundle(ImageBundle {
//             style: button_icon_style.clone(),
//             image: UiImage(icon),
//             ..Default::default()
//           });
//           parent.spawn_bundle(TextBundle {
//             text: Text::with_section("New Game", button_text_style.clone(), Default::default()),
//             ..Default::default()
//           });
//         });
//       parent
//         .spawn_bundle(ButtonBundle {
//           style: button_style,
//           color: NORMAL_BUTTON.into(),
//           ..Default::default()
//         })
//         .insert(MenuButtonAction::Quit)
//         .with_children(|parent| {
//           let icon = asset_server.load("exitRight.png");
//           parent.spawn_bundle(ImageBundle {
//             style: button_icon_style,
//             image: UiImage(icon),
//             ..Default::default()
//           });
//           parent.spawn_bundle(TextBundle {
//             text: Text::with_section("Quit", button_text_style, Default::default()),
//             ..Default::default()
//           });
//         });
//     });
// }