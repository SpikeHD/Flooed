pub mod json_helpers;
pub mod logger;
pub mod open;
pub mod paths;
pub mod process;
pub mod ws;

pub fn register_path_commands(ws: &mut ws::WsConnector) {
  ws.register_command("get_plugin_dir", |_| {
    Some(
      paths::get_plugin_dir()
        .to_str()
        .unwrap_or_default()
        .to_string(),
    )
  });

  ws.register_command("get_theme_dir", |_| {
    Some(
      paths::get_theme_dir()
        .to_str()
        .unwrap_or_default()
        .to_string(),
    )
  });

  ws.register_command("open_plugins", |_| {
    open::open_folder(paths::get_plugin_dir()).unwrap_or_default();
    None
  });

  ws.register_command("open_themes", |_| {
    open::open_folder(paths::get_theme_dir()).unwrap_or_default();
    None
  });
}
