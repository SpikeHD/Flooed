use serde::{Deserialize, Serialize};
use rsrpc::{
  detection::{DetectableActivity, Executable},
  RPCServer,
};
use std::sync::{Arc, Mutex};
use sysinfo::System;
use window_titles::ConnectionTrait;

use crate::util::paths::custom_detectables_path;

#[derive(Clone, Deserialize)]
struct Payload {
  name: String,
  exe: String,
}

#[derive(Serialize, Deserialize)]
pub struct Window {
  title: String,
  process_name: String,
  pid: u32,
}

pub fn get_local_detectables() -> Vec<DetectableActivity> {
  let path = custom_detectables_path();

  // Make if doesn't exist
  if !path.exists() {
    std::fs::write(path, "[]").unwrap_or_default();
    return vec![];
  }

  let contents = std::fs::read_to_string(path).unwrap_or_default();
  let detectables: Vec<DetectableActivity> = serde_json::from_str(&contents).unwrap_or_default();

  detectables
}

pub fn append_to_local(detectables: Vec<DetectableActivity>) {
  let mut local_detectables = get_local_detectables();

  local_detectables.extend(detectables);

  let path = custom_detectables_path();

  // Write back to file
  std::fs::write(
    path,
    serde_json::to_string(&local_detectables).unwrap_or_default(),
  )
  .unwrap_or_default();
}

pub fn start_rpc_server() {
  let detectable = reqwest::blocking::get(
    "https://gist.githubusercontent.com/SpikeHD/209bd9b17c97f45dc5be4803c748726f/raw/ddf8ed33621933b4e3c58cf1113e1679ab9fd9b5/dorion_detectable.json",
  )
  .expect("Request for detectable.json failed")
  .text()
  .expect("Failed to get text from response");

  // This accepts both a `&str` or a `String`
  let server = Arc::new(Mutex::new(
    RPCServer::from_json_str(detectable).expect("Failed to start RPC server"),
  ));

  // When the "add_detectable" event is emitted, add the detectable to the server
  // TODO event to add detectable

  // TODO event to remove detectable

  server.lock().unwrap().start();

  // Add any local custom detectables
  server
    .lock()
    .unwrap()
    .append_detectables(get_local_detectables());

  loop {
    std::thread::sleep(std::time::Duration::from_millis(10));
  }
}

fn blank_activity() -> DetectableActivity {
  serde_json::from_str::<DetectableActivity>(
    r#"
  {
    "bot_public": true,
    "bot_require_code_grant": false,
    "description": "",
    "executables": [],
    "name": "",
    "flags": 0,
    "hook": true,
    "id": "1337",
    "summary": "",
    "type": 1
  }
  "#,
  )
  .unwrap()
}

pub fn get_windows() -> Vec<Window> {
  let conn = window_titles::Connection::new().expect("Failed to connect to window titles");
  let mut system = System::new_all();

  system.refresh_processes();

  let windows: Vec<Window> = conn
    .window_titles()
    .unwrap_or_default()
    .into_iter()
    .map(|w| {
      let proc = system.process(sysinfo::Pid::from_u32(w.pid));
      let process_name = if let Some(proc) = proc {
        proc.name().to_string()
      } else {
        format!("Unknown ({})", w.pid)
      };

      Window {
        title: w.title,
        pid: w.pid,
        process_name,
      }
    })
    .collect();

  windows
}
