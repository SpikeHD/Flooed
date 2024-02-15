use serde_json::Value;

pub mod plugins;
pub mod themes;

pub fn register_plugin_commands(ws: &mut super::util::ws::WsConnector) {
  ws.register_command("get_new_plugins", |_| {
    plugins::get_new_plugins();
    None
  });

  ws.register_command("get_plugins", |_| {
    let plugins = plugins::get_plugin_list();
    Some(
      serde_json::to_value(plugins)
        .unwrap_or_default()
        .to_string(),
    )
  });

  ws.register_command("toggle_plugin", |data| {
    let name = data
      .unwrap_or(Value::Null)
      .as_str()
      .unwrap_or_default()
      .to_string();
    Some(
      serde_json::to_value(plugins::toggle_plugin(name))
        .unwrap_or_default()
        .to_string(),
    )
  });

  ws.register_command("toggle_preload", |data| {
    let name = data
      .unwrap_or(Value::Null)
      .as_str()
      .unwrap_or_default()
      .to_string();
    Some(
      serde_json::to_value(plugins::toggle_preload(name))
        .unwrap_or_default()
        .to_string(),
    )
  });
}

pub fn register_theme_commands(_ws: &mut super::util::ws::WsConnector) {
  // TODO
}
