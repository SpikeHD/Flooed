#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod config;
mod util;

use config::get_config;
use util::logger;
use util::ws::WsConnector;
use webui_rs::webui::{wait, WebUIBrowser, Window};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

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
  win.add_extension(ext.to_str().unwrap());

  // Start the websocket connector
  let mut ws = WsConnector::new();

  register_commands(&mut ws);

  ws.start();

  // Start the browser
  if !win.show_browser(client, WebUIBrowser::ChromiumBased) {
    // If no chromium based browser is found, just show with whatever is available
    win.show(client);

    show_notification("Flooed", "No chromium based browser found, using default browser. Note that you may encounter limited functionality.")
  }

  wait();
}

fn show_notification(summary: &str, body: &str) {
  notify_rust::Notification::new()
    .summary(summary)
    .body(body)
    .icon("discord")
    .show()
    // This can fail, it's not really vital
    .unwrap_or(())
}

fn register_commands(ws: &mut WsConnector) {
  ws.register_command("get_version", |_| VERSION.to_string());
  ws.register_command("read_config_file", |_| config::read_config_file());
}
