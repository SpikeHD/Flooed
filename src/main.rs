#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod compat;
mod config;
mod util;

use std::fs;

use config::get_config;
use util::{logger, paths::get_profile_dir};
use util::ws::WsConnector;
//use webui_rs::webui::bindgen::webui_get_best_browser;
use webui_rs::webui::{bindgen::webui_set_profile, wait, WebUIBrowser, Window};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  logger::init(true);
  let config = get_config();
  let mut win = Window::new();
  let browser = WebUIBrowser::ChromiumBased;
  //let browser = unsafe { WebUIBrowser::from_usize(webui_get_best_browser(win.id)) };

  // Ensure profile dir exists
  let profile_dir = get_profile_dir(browser.to_usize());
    
  if fs::metadata(&profile_dir).is_err() {
    println!("Creating profile dir: {:?}", profile_dir);
    fs::create_dir_all(&profile_dir).expect("Failed to create profile dir");
  }

  logger::log(format!("Profile dir: {:?}", profile_dir));

  // Set the profile dir
  unsafe {
    let path_cstr = std::ffi::CString::new(
      profile_dir.to_str().expect("Failed to convert profile dir to string"),
    ).expect("Failed to convert profile dir to CString");

    webui_set_profile(
      win.id,
      "Flooed".as_ptr() as *const i8,
      path_cstr.as_ptr() as *const i8,
    );
  }

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

  if browser == WebUIBrowser::Firefox {
    unsafe {
      compat::firefox::move_firefox_extension();
    }
  } else {
    // Append ./ext
    let ext = cwd.join("ext").join("mv2");
    win.add_extension(ext.to_str().unwrap());
  }

  // Start the websocket connector
  let mut ws = WsConnector::new();

  register_commands(&mut ws);

  ws.start();

  // Start the browser
  win.show_browser(client, browser);

  wait();
}

fn show_notification(summary: &str, body: &str) {
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

fn register_commands(ws: &mut WsConnector) {
  ws.register_command("get_version", |_| VERSION.to_string());
  ws.register_command("read_config_file", |_| config::read_config_file());
}
