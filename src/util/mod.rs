use crate::register_command;

pub mod logger;
pub mod macros;
pub mod open;
pub mod paths;
pub mod process;
pub mod ws;

pub fn register_path_commands(ws: &mut ws::WsConnector) {
  register_command!(ws, get_plugin_dir, |_| {
    Some(
      paths::get_plugin_dir()
        .to_str()
        .unwrap_or_default()
        .to_string(),
    )
  });

  register_command!(ws, get_theme_dir, |_| {
    Some(
      paths::get_theme_dir()
        .to_str()
        .unwrap_or_default()
        .to_string(),
    )
  });

  register_command!(ws, open_plugins, |_| {
    open::open_folder(paths::get_plugin_dir()).unwrap_or_default();
    None
  });

  register_command!(ws, open_themes, |_| {
    open::open_folder(paths::get_theme_dir()).unwrap_or_default();
    None
  });
}
