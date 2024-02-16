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

pub fn register_theme_commands(ws: &mut super::util::ws::WsConnector) {
  ws.register_command("get_theme", |data| {
    if let Some(data) = data {
      let name = data.get("name").unwrap_or(&Value::Null).as_str().unwrap().to_string();
      let theme = themes::get_theme(name);

      println!("{:?}", theme);

      if let Ok(theme) = theme {
        Some(
          serde_json::to_value(theme)
            .unwrap_or_default()
            .to_string(),
        )
      } else {
        None
      }
    } else {
      None
    }
  });

  ws.register_command("get_theme_names", |_| {
    if let Ok(themes) = themes::get_theme_names() {
      Some(
        serde_json::to_value(themes)
          .unwrap_or_default()
          .to_string(),
      )
    } else {
      None
    }
  });

  ws.register_command("theme_from_link", |data| {
    if let Some(data) = data {
      let link = data.as_str().unwrap_or_default().to_string();
      
      Some(
        serde_json::to_value(themes::theme_from_link(link))
          .unwrap_or_default()
          .to_string(),
      )
    } else {
      None
    }
  });

  // This is a Dorion compat command
  ws.register_command("localize_imports", |data| {
    if let Some(data) = data {
      let css = match data.get("css") {
        Some(c) => c.as_str().unwrap().to_string(),
        None => return Some(String::from("")),
      };
      
      Some(css)
    } else {
      None
    }
  });
}
