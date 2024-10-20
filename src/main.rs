#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod compat;
mod config;
mod extra;
mod util;

use std::fs;

use config::get_config;
use crowser::{RemoteConfig, Window, WindowIpc};
use dialog::DialogBox;
use util::logger::log;
use util::process::process_already_exists;
use util::{logger, paths::get_profile_dir};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  logger::init(true);

  if process_already_exists() {
    // TODO We'll do the deep link thing later
    std::process::exit(0);
  }

  let config = get_config();

  // Ensure profile dir exists
  let profile_dir = get_profile_dir();

  if fs::metadata(&profile_dir).is_err() {
    logger::log(format!("Creating profile dir: {:?}", profile_dir));
    fs::create_dir_all(&profile_dir).expect("Failed to create profile dir");
  }

  logger::log(format!("Profile dir: {:?}", profile_dir));

  let client_type = config.client_type.unwrap_or("default".to_string());
  let client = match client_type.as_str() {
    "default" => "https://discord.com/app",
    "canary" => "https://canary.discord.com/app",
    "ptb" => "https://ptb.discord.com/app",
    _ => "https://discord.com/app",
  };

  logger::log("Starting on client: ".to_string() + client);

  let win_config = RemoteConfig { url: client.to_string() };
  let mut win = Window::new(win_config, None, profile_dir).unwrap_or_else(|e| {
    dialog::Message::new(format!("Error creating Flooed window: {}", e))
      .title("Error")
      .show()
      .unwrap();

    std::process::exit(1);
  });

  win.set_initialization_script(
    format!("{}", extra::client_mod::load_mods_js())
  ).unwrap_or_default();

  // Start RPC server
  if config.rpc_server.unwrap_or(false) {
    std::thread::spawn(|| {
      extra::rpc::start_rpc_server();
    });
  }
  
  win.create().unwrap_or_else(|e| {
    dialog::Message::new(format!("Error creating Flooed window: {}", e))
      .title("Error")
      .show()
      .unwrap();
  })
}

fn _show_notification(summary: &str, body: &str) {
  let n = notify_rust::Notification::new()
    .summary(summary)
    .body(body)
    .icon("discord")
    .show();

  match n {
    Ok(_) => {}
    Err(e) => {
      logger::log(format!("Failed to show notification: {}", e));
    }
  }
}

fn register_commands(ipc: &WindowIpc) {

}
