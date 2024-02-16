use miniserde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::util::{logger::log, paths::get_plugin_dir};

#[derive(Serialize, Deserialize)]
pub struct PluginDetails {
  name: String,
  enabled: bool,
  preload: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PluginList {
  plugins: HashMap<String, PluginDetails>,
}

pub fn get_new_plugins() {
  let plugins_dir = get_plugin_dir();
  let mut plugins_list = get_plugin_list();

  let plugins = fs::read_dir(&plugins_dir).unwrap();

  // Not only do we add plugins, but we also remove plugins that don't exist anymore
  for plugin in plugins {
    let plugin = plugin.unwrap();
    let filename = plugin.file_name().to_str().unwrap().to_string();
    let mut plugin_name = filename.clone();

    if !plugin_name.ends_with(".js") {
      continue;
    }

    // Plugin name without the .js
    plugin_name = plugin_name.split('.').next().unwrap().to_string();

    let plugin_details = plugins_list.plugins.get(&filename);

    if plugin_details.is_none() {
      plugins_list
        .plugins
        .insert(
          filename,
          PluginDetails {
            name: plugin_name,
            enabled: false,
            preload: false,
          },
        );
    }
  }

  let plugins_to_remove: Vec<String> = plugins_list
    .plugins
    .keys()
    .filter(|&plugin_name| {
      let plugin_path = plugins_dir.join(plugin_name);
      !plugin_path.exists()
    })
    .cloned()
    .collect();

  for plugin_name in plugins_to_remove {
    plugins_list.plugins.remove(&plugin_name);
  }

  write_plugins_json(plugins_list.plugins);

  log("Plugins updated");
}

fn write_plugins_json(list: HashMap<String, PluginDetails>) {
  let plugins_dir = get_plugin_dir();
  let plugins_json = plugins_dir.join("plugins.json");

  let plugins_str = miniserde::json::to_string(&list);

  // If it's empty, something got borked, so just write an empty array
  if plugins_str.is_empty() {
    fs::write(plugins_json, "[]").unwrap();
    return;
  }

  fs::write(plugins_json, plugins_str).unwrap();
}

pub fn get_plugin_list() -> PluginList {
  let plugins_dir = get_plugin_dir();
  let plugins_json = plugins_dir.join("plugins.json");

  if !plugins_json.exists() {
    // Create the plugins list file
    log("Plugins.json does not exit, recreating...");
    fs::write(plugins_json, "[]").unwrap();

    return PluginList { plugins: HashMap::new() };
  }

  let plugins_json = fs::read_to_string(plugins_json).unwrap_or_default();
  let plugins_json: PluginList = miniserde::json::from_str(&plugins_json)
    .unwrap_or_else(|_| {
      log("Plugins.json invalid, recreating...");
      fs::write(plugins_json, "{}").unwrap_or_default();

      PluginList { plugins: HashMap::new() }
    });

  plugins_json
}

pub fn toggle_plugin(name: String) -> bool {
  let mut plugins_list = get_plugin_list();
  let mut found = false;

  plugins_list.plugins.iter_mut().for_each(|p| {
    if p.0.as_str() == name {
      p.1.enabled = !p.1.enabled;
      found = true;
    }
  });

  write_plugins_json(plugins_list.plugins);

  if !found {
    log(format!("Plugin {} not found", name).as_str());
  }

  found
}

pub fn toggle_preload(name: String) -> bool {
  let mut plugins_list = get_plugin_list();
  let mut found = false;

  plugins_list.plugins.iter_mut().for_each(|p| {
    if p.0.as_str() == name {
      p.1.preload = !p.1.preload;
      found = true;
    }
  });

  write_plugins_json(plugins_list.plugins);

  if !found {
    log(format!("Plugin {} not found", name).as_str());
  }

  found
}
