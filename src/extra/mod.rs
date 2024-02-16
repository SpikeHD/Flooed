use miniserde::json::Value;

use crate::util::json_helpers::{ToObject, ToString};

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
      miniserde::json::to_string(&plugins)
    )
  });

  ws.register_command("toggle_plugin", |data| {
    let name = data
      .unwrap_or(Value::Null)
      .to_string();

    Some(
      plugins::toggle_plugin(name).to_string()
    )
  });

  ws.register_command("toggle_preload", |data| {
    let name = data
      .unwrap_or(Value::Null)
      .to_string();

    Some(
      plugins::toggle_preload(name).to_string()
    )
  });
}

pub fn register_theme_commands(ws: &mut super::util::ws::WsConnector) {
  ws.register_command("get_theme", |data| {
    if let Some(data) = data {
      let data = data.to_object();
      let name = data.get("name").unwrap_or(&Value::Null).to_string();
      let theme = themes::get_theme(name);

      println!("{:?}", theme);

      if let Ok(theme) = theme {
        Some(
          theme
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
      // Convert the Vec<String> to a a Vec<Value>
      let themes: Vec<Value> = themes.iter().map(|t| Value::String(t.to_string())).collect();
      let arr = miniserde::json::Array::from_iter(themes.into_iter());

      Some(
        miniserde::json::to_string(&arr)
      )
    } else {
      None
    }
  });

  ws.register_command("theme_from_link", |data| {
    if let Some(data) = data {
      let data = data.to_object();
      let name = data.get("link").unwrap_or(&Value::Null).to_string();
      
      Some(
        name
      )
    } else {
      None
    }
  });

  // This is a Dorion compat command
  ws.register_command("localize_imports", |data| {
    if let Some(data) = data {
      let data = data.to_object();
      let css = data.get("css").unwrap_or(&Value::Null).to_string();
      
      Some(css)
    } else {
      None
    }
  });
}
