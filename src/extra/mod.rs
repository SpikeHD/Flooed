use std::sync::Arc;

use serde_json::Value;

use crate::register_command;

pub mod client_mod;
pub mod plugins;
pub mod rpc;
pub mod themes;

pub fn register_plugin_commands(ws: &mut super::util::ws::WsConnector) {
  register_command!(ws, get_new_plugins, |_| {
    plugins::get_new_plugins();
    None
  });

  register_command!(
    ws,
    get_plugins,
    Box::new(|_| {
      let plugins = plugins::get_plugin_list();
      Some(
        serde_json::to_value(plugins)
          .unwrap_or_default()
          .to_string(),
      )
    })
  );

  register_command!(ws, toggle_plugin, |data: Option<Value>| {
    let name = data
      .unwrap_or(Value::Null)
      .as_str()
      .unwrap_or_default()
      .to_string();
    Some(plugins::toggle_plugin(name).to_string())
  });

  register_command!(ws, toggle_preload, |data: Option<Value>| {
    let name = data
      .unwrap_or(Value::Null)
      .as_str()
      .unwrap_or_default()
      .to_string();
    Some(plugins::toggle_preload(name).to_string())
  });
}

pub fn register_client_mod_commands(ws: &mut super::util::ws::WsConnector) {
  // Preload the client mods
  let client_mod_js = Arc::new(client_mod::load_mods_js());
  let client_mod_css = Arc::new(client_mod::load_mods_css());

  register_command!(ws, available_mods, |_| {
    let mods = client_mod::available_mods();
    Some(serde_json::to_value(mods).unwrap_or_default().to_string())
  });

  register_command!(ws, load_client_mods_js, move |_| {
    Some((*client_mod_js).clone())
  });

  register_command!(ws, load_client_mods_css, move |_| {
    Some((*client_mod_css).clone())
  });
}

pub fn register_theme_commands(ws: &mut super::util::ws::WsConnector) {
  register_command!(ws, get_theme, |data: Option<Value>| {
    if let Some(data) = data {
      let name = data
        .get("name")
        .unwrap_or(&Value::Null)
        .as_str()
        .unwrap_or("")
        .to_string();
      let theme = themes::get_theme(name);

      println!("{:?}", theme);

      if let Ok(theme) = theme {
        Some(serde_json::to_value(theme).unwrap_or_default().to_string())
      } else {
        None
      }
    } else {
      None
    }
  });

  register_command!(ws, get_theme_names, |_| {
    if let Ok(themes) = themes::get_theme_names() {
      Some(serde_json::to_value(themes).unwrap_or_default().to_string())
    } else {
      None
    }
  });

  register_command!(ws, theme_from_link, |data: Option<Value>| {
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
  register_command!(ws, localize_imports, |data: Option<Value>| {
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
