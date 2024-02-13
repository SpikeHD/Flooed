#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod config;
mod util;

use config::get_config;
use util::logger;
use webui_rs::webui::{wait, Window, WebUIBrowser};

fn main() {
  logger::init(true);
  let config = get_config();
  let mut win = Window::new();

  let client = match config.client_type.unwrap_or("default".to_string()).as_str() {
    "default" => "https://discord.com/app",
    "canary" => "https://canary.discord.com/app",
    "ptb" => "https://ptb.discord.com/app",
    _ => "https://discord.com/app",
  };

  unsafe {
    webui_rs::webui::bindgen::webui_set_port(win.id, 10100);
  }

  logger::log("Starting on client: ".to_string() + client);

  // Get current working dir
  let cwd = std::env::current_dir().unwrap();
  // Append ./ext
  let ext = cwd.join("ext").join("mv2");

  // This should load our extension
  win.add_extension(ext.to_str().unwrap().to_string());

  win.show_browser(client, WebUIBrowser::ChromiumBased);

  wait();
}
