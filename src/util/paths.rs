use std::{fs, path::PathBuf};

use crate::config::get_config;
use crate::util::logger::log;

pub fn get_config_dir() -> PathBuf {
  // First check for a local config file
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if fs::metadata(&local_config_dir).is_ok() {
    return local_config_dir;
  }

  log("No local config file found. Using default.");

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let config_file = appdata.join("flooed").join("config.json");

  if fs::metadata(appdata.join("flooed")).is_err() {
    fs::create_dir_all(appdata.join("flooed")).expect("Error creating appdata dir");
  }

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      &config_file,
      r#"{ "theme": "none", "zoom": "1.0", "client_type": "default", "sys_tray": false, "block_telemetry": false }"#,
    )
    .unwrap_or(());
  }

  config_file
}

pub fn config_is_local() -> bool {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  fs::metadata(local_config_dir).is_ok()
}

pub fn get_plugin_dir() -> std::path::PathBuf {
  // First check for a local plugin dir
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_plugin_dir = current_exe.parent().unwrap().join("plugins");

  if fs::metadata(&local_plugin_dir).is_ok() {
    return local_plugin_dir;
  }

  log("No local plugin dir found. Using default.");

  #[cfg(target_os = "windows")]
  let plugin_dir = dirs::home_dir()
    .unwrap_or_default()
    .join("flooed")
    .join("plugins");

  #[cfg(not(target_os = "windows"))]
  let plugin_dir = dirs::config_dir()
    .unwrap_or_default()
    .join("flooed")
    .join("plugins");

  if fs::metadata(&plugin_dir).is_err() {
    match fs::create_dir_all(&plugin_dir) {
      Ok(()) => (),
      Err(e) => {
        log(format!("Error creating plugins dir: {}", e));
        return plugin_dir;
      }
    };
  }

  plugin_dir
}

pub fn get_theme_dir() -> std::path::PathBuf {
  // First see if there is a local theme dir
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_theme_dir = current_exe.parent().unwrap().join("themes");

  if fs::metadata(&local_theme_dir).is_ok() {
    return local_theme_dir;
  }

  log("No local theme dir found. Using default.");

  #[cfg(target_os = "windows")]
  let theme_dir = dirs::home_dir()
    .unwrap_or_default()
    .join("flooed")
    .join("themes");

  #[cfg(not(target_os = "windows"))]
  let theme_dir = dirs::config_dir()
    .unwrap_or_default()
    .join("flooed")
    .join("themes");

  if fs::metadata(&theme_dir).is_err() {
    match fs::create_dir_all(&theme_dir) {
      Ok(()) => (),
      Err(e) => {
        log(format!("Error creating theme dir: {}", e));
        return theme_dir;
      }
    };
  }

  // Also create theme cache dir
  let cache_dir = theme_dir.join("cache");

  if fs::metadata(&cache_dir).is_err() {
    match fs::create_dir_all(&cache_dir) {
      Ok(()) => (),
      Err(e) => {
        log(format!("Error creating theme cache dir: {}", e));
        return theme_dir;
      }
    };
  }

  theme_dir
}

pub fn custom_detectables_path() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if fs::metadata(local_config_dir).is_ok() {
    // This is a portable install, so we can use the local injection dir
    return current_exe.parent().unwrap().join("detectables.json");
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  appdata.join("flooed").join("detectables.json")
}

pub fn log_file_path() -> PathBuf {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if fs::metadata(local_config_dir).is_ok() {
    // This is a portable install, so we can use the local injection dir
    return current_exe
      .parent()
      .unwrap()
      .join("logs")
      .join("latest.log");
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  appdata.join("flooed").join("logs").join("latest.log")
}
