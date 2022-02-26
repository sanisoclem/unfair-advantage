use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};
use std::collections::VecDeque;

pub enum DebugCmdEvents {
  Print(String),
  Unknown(String),
  Exit,
}

#[derive(Default)]
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<DebugConsole>()
      .add_event::<DebugCmdEvents>()
      .add_system(debug_gui)
      .add_system(cmd_handler);
  }
}

#[derive(Default)]
struct DebugConsole {
  console_open: bool,
  cmd_buffer: String,
  cmd_history: VecDeque<String>,
  output: VecDeque<String>,
}

impl DebugConsole {
  pub fn invoke(&mut self, mut event_writer: EventWriter<DebugCmdEvents>) {
    if self.cmd_buffer.is_empty() {
      return;
    }
    let cur_cmd = std::mem::replace(&mut self.cmd_buffer, String::default());
    let evt = match cur_cmd.as_str() {
      "exit" | "quit" | "q" => DebugCmdEvents::Exit,
      _ => DebugCmdEvents::Unknown(cur_cmd.clone()),
    };

    println!("Invocing cmd: {:?}", cur_cmd);

    event_writer.send(evt);
    self.cmd_history.push_front(cur_cmd);
  }
}

fn dark_light_mode_switch(ui: &mut egui::Ui) {
  let style: egui::Style = (*ui.ctx().style()).clone();
  let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
  if let Some(visuals) = new_visuals {
    ui.ctx().set_visuals(visuals);
  }
}

fn debug_gui(
  mut egui_context: ResMut<EguiContext>,
  mut dbg_state: ResMut<DebugConsole>,
  dbg_events: EventWriter<DebugCmdEvents>,
) {
  let ctx = egui_context.ctx_mut();
  let toggle_console = ctx.input().key_pressed(egui::Key::Tab);

  if toggle_console {
    dbg_state.console_open = !dbg_state.console_open
  }

  egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
    ui.horizontal_wrapped(|ui| {
      dark_light_mode_switch(ui);

      ui.checkbox(&mut dbg_state.console_open, "💻 Console");
      ui.separator();
      egui::warn_if_debug_build(ui);
    });
  });

  if dbg_state.console_open {
    egui::SidePanel::left("Console").show(ctx, |ui| {
      let cmd_te = ui.add(
        egui::TextEdit::singleline(&mut dbg_state.cmd_buffer)
          .hint_text("Enter command")
          .cursor_at_end(true)
          .desired_width(ui.available_width()),
      );
      if cmd_te.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
        dbg_state.invoke(dbg_events);
        cmd_te.request_focus();
      } else if toggle_console && dbg_state.console_open {
        cmd_te.request_focus();
      }
      ui.separator();
      egui::ScrollArea::vertical().show(ui, |ui| {
        ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);

        for entry in &dbg_state.output {
          ui.label(entry);
        }
      });
    });
  }
}

fn cmd_handler(
  mut events: EventReader<DebugCmdEvents>,
  mut app_exit_events: EventWriter<AppExit>,
  mut dbg_state: ResMut<DebugConsole>,
) {
  for evt in events.iter() {
    match evt {
      DebugCmdEvents::Exit => app_exit_events.send(AppExit),
      DebugCmdEvents::Unknown(cmd) => {
        dbg_state
          .output
          .push_front(format!("Unknown command: {}", cmd));
      }
      DebugCmdEvents::Print(msg) => {
        dbg_state.output.push_front(msg.clone());
      }
    }
  }
}
