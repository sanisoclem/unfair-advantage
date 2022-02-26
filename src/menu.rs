use super::{systems::cleanup_system, GameState};

use bevy::{app::AppExit, prelude::*};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state(MenuState::Disabled)
      .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
      .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup))
      .add_system_set(
        SystemSet::on_exit(MenuState::Main).with_system(cleanup_system::<OnMainMenuScreen>),
      )
      // Common systems to all screens that handles buttons behaviour
      .add_system_set(
        SystemSet::on_update(GameState::Menu)
          .with_system(menu_action)
          .with_system(button_system),
      );
  }
}

// State used for the current menu screen
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
  Main,
  Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
  Play,
  Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut UiColor, Option<&SelectedOption>),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color, selected) in interaction_query.iter_mut() {
    *color = match (*interaction, selected) {
      (Interaction::Clicked, _) => PRESSED_BUTTON.into(),
      (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
      (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
      (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
      (Interaction::None, None) => NORMAL_BUTTON.into(),
    }
  }
}

fn menu_setup(mut menu_state: ResMut<State<MenuState>>) {
  let _ = menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("Shizuru-Regular.ttf");
  // Common style for all buttons on the screen
  let button_style = Style {
    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
    margin: Rect::all(Val::Px(20.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Default::default()
  };
  let button_icon_style = Style {
    size: Size::new(Val::Px(30.0), Val::Auto),
    // This takes the icons out of the flexbox flow, to be positionned exactly
    position_type: PositionType::Absolute,
    // The icon will be close to the left border of the button
    position: Rect {
      left: Val::Px(10.0),
      right: Val::Auto,
      top: Val::Auto,
      bottom: Val::Auto,
    },
    ..Default::default()
  };
  let button_text_style = TextStyle {
    font: font.clone(),
    font_size: 40.0,
    color: TEXT_COLOR,
  };

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
    .insert(OnMainMenuScreen)
    .with_children(|parent| {
      // Display the game name
      parent.spawn_bundle(TextBundle {
        style: Style {
          margin: Rect::all(Val::Px(50.0)),
          ..Default::default()
        },
        text: Text::with_section(
          "Main Menu UI",
          TextStyle {
            font: font.clone(),
            font_size: 80.0,
            color: TEXT_COLOR,
          },
          Default::default(),
        ),
        ..Default::default()
      });

      parent
        .spawn_bundle(ButtonBundle {
          style: button_style.clone(),
          color: NORMAL_BUTTON.into(),
          ..Default::default()
        })
        .insert(MenuButtonAction::Play)
        .with_children(|parent| {
          let icon = asset_server.load("right.png");
          parent.spawn_bundle(ImageBundle {
            style: button_icon_style.clone(),
            image: UiImage(icon),
            ..Default::default()
          });
          parent.spawn_bundle(TextBundle {
            text: Text::with_section("New Game", button_text_style.clone(), Default::default()),
            ..Default::default()
          });
        });
      parent
        .spawn_bundle(ButtonBundle {
          style: button_style,
          color: NORMAL_BUTTON.into(),
          ..Default::default()
        })
        .insert(MenuButtonAction::Quit)
        .with_children(|parent| {
          let icon = asset_server.load("exitRight.png");
          parent.spawn_bundle(ImageBundle {
            style: button_icon_style,
            image: UiImage(icon),
            ..Default::default()
          });
          parent.spawn_bundle(TextBundle {
            text: Text::with_section("Quit", button_text_style, Default::default()),
            ..Default::default()
          });
        });
    });
}

fn menu_action(
  interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
  mut app_exit_events: EventWriter<AppExit>,
  mut menu_state: ResMut<State<MenuState>>,
  mut game_state: ResMut<State<GameState>>,
) {
  for (interaction, menu_button_action) in interaction_query.iter() {
    if *interaction == Interaction::Clicked {
      match menu_button_action {
        MenuButtonAction::Quit => app_exit_events.send(AppExit),
        MenuButtonAction::Play => {
          game_state.set(GameState::Game).unwrap();
          menu_state.set(MenuState::Disabled).unwrap();
        }
      }
    }
  }
}
