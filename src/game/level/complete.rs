use bevy::prelude::*;

#[derive(Component)]
pub struct CompleteTimer {
  pub timer: Timer
}

#[derive(Component)]
pub struct CompleteLoadingTag;

pub struct CompletedLevels {
  pub count: u32,
  pub messages: Vec<String>
}
impl Default for CompletedLevels {
  fn default() -> Self {
    CompletedLevels {
      count: 0,
      messages: vec![
        "What, there's another level!?".to_owned(),
        "I didn't have time to finish the boss fight...".to_owned(),
        "You're getting good at this!".to_owned(),
        "Github co-pilot wrote 20% of this \"game\"".to_owned(),
        "Thanks for playing!".to_owned(),
        "There's nothing else to see...".to_owned(),
        "Seriously, it's ".to_owned()
        ]
    }
  }
}

pub fn reset_level_count(mut level_count: ResMut<CompletedLevels>) {
  level_count.count = 0;
}

pub fn show_complete(mut commands: Commands, asset_server: Res<AssetServer>, mut level_count: ResMut<CompletedLevels>) {
  let font = asset_server.load("Shizuru-Regular.ttf");
  let msg_index = level_count.count as usize % level_count.messages.len();
  level_count.count += 1;


  commands.spawn().insert(CompleteLoadingTag).insert(CompleteTimer {timer: Timer::from_seconds(5.0, false)});

  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        margin: Rect::all(Val::Auto),
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        ..Default::default()
      },
      color: Color::CRIMSON.into(),
      ..Default::default()
    })
    .insert(CompleteLoadingTag)
    .with_children(|parent| {
      parent.spawn_bundle(TextBundle {
        style: Style {
          margin: Rect::all(Val::Px(50.0)),
          ..Default::default()
        },
        text: Text::with_section(
          level_count.messages[msg_index].clone(),
          TextStyle {
            font: font.clone(),
            font_size: 80.0,
            color: Color::rgb(0.9, 0.9, 0.9),
          },
          Default::default(),
        ),
        ..Default::default()
      });
    });
}

pub fn wait_to_load_next_level(time: Res<Time>, mut qry: Query<&mut CompleteTimer>, mut level_state: ResMut<State<super::LevelState>>) {
  for mut i in qry.iter_mut() {
    i.timer.tick(time.delta());
    if i.timer.just_finished() {
      level_state.set(super::LevelState::Loaded).expect("Failed to set level state");
    }
  }

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
