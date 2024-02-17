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
use extra::{register_plugin_commands, register_theme_commands};
use util::process::process_already_exists;
use util::register_path_commands;
use util::ws::WsConnector;
use util::{logger, paths::get_profile_dir};
//use webui_rs::webui::bindgen::webui_get_best_browser;
use webui_rs::webui::{bindgen::webui_set_profile, wait, WebUIBrowser, Window};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  logger::init(true);

  if process_already_exists() {
    // TODO We'll do the deep link thing later
    std::process::exit(0);
  }

  let config = get_config();
  let mut win = Window::new();
  let browser = WebUIBrowser::ChromiumBased;
  //let browser = unsafe { WebUIBrowser::from_usize(webui_get_best_browser(win.id)) };

  // Ensure profile dir exists
  let profile_dir = get_profile_dir(browser.to_usize());

  if fs::metadata(&profile_dir).is_err() {
    logger::log(format!("Creating profile dir: {:?}", profile_dir));
    fs::create_dir_all(&profile_dir).expect("Failed to create profile dir");
  }

  logger::log(format!("Profile dir: {:?}", profile_dir));

  // Set the profile dir
  unsafe {
    let path_cstr = std::ffi::CString::new(
      profile_dir
        .to_str()
        .expect("Failed to convert profile dir to string"),
    )
    .expect("Failed to convert profile dir to CString");

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

  // Start RPC server
  if config.rpc_server.unwrap_or(false) {
    std::thread::spawn(|| {
      extra::rpc::start_rpc_server();
    });
  }

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

  // We should be able to safely wait until the websocket server has no clients
  loop {
    if ws.clients.lock().unwrap().is_empty() {
      break;
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
  }
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

fn register_commands(ws: &mut WsConnector) {
  ws.register_command("get_version", |_| Some(VERSION.to_string()));
  ws.register_command("read_config_file", |_| Some(config::read_config_file()));
  ws.register_command("write_config_file", |data| {
    if let Some(data) = data {
      let contents = match data.get("contents") {
        Some(c) => c.as_str().unwrap().to_string(),
        None => return Some(String::from("false")),
      };
      config::write_config_file(contents);
      return Some(String::from("true"));
    }

    Some(String::from("false"))
  });

  ws.register_command("relaunch", |_| {
    std::process::Command::new(std::env::current_exe().unwrap())
      .spawn()
      .expect("Failed to relaunch");

    std::process::exit(0);
  });

  register_plugin_commands(ws);
  register_theme_commands(ws);
  register_path_commands(ws);
}
