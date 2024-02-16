use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_websockets::{self, Event, Message, Responder};
use std::{collections::HashMap, sync::{Arc, Mutex}};

use super::logger;

#[derive(Serialize, Deserialize)]
struct Command {
  command: String,
  data: Option<Value>,
  id: u64,
}

type CallbackFn = fn(Option<Value>) -> Option<String>;

pub struct WsConnector {
  pub clients: Arc<Mutex<HashMap<u64, Responder>>>,
  
  ws: Arc<simple_websockets::EventHub>,
  commands: HashMap<String, CallbackFn>,
}

impl WsConnector {
  pub fn new() -> WsConnector {
    WsConnector {
      ws: Arc::new(
        simple_websockets::launch(10102)
          .expect("Could not launch WebSocket server. Is Flooed already running?"),
      ),
      commands: HashMap::new(),
      clients: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /**
   * Start the WebSocket server. Commands cannot be registered after this is called.
   */
  pub fn start(&self) {
    let ws = self.ws.clone();
    let commands = self.commands.clone();
    let mut clients = self.clients.clone();

    std::thread::spawn(move || {
      loop {
        let mut clients = clients.lock().unwrap();

        match ws.poll_event() {
          Event::Connect(client_id, responder) => {
            logger::log(format!("Flooed frontend connected: {}", client_id));

            // Only insert if there isn't already an active client
            if clients.is_empty() {
              clients.insert(client_id, responder);
            } else {
              logger::log(format!("Flooed frontend already connected: {}", client_id));
            }
          }
          Event::Disconnect(client_id) => {
            logger::log(format!("Flooed frontend disconnected: {}", client_id));
            clients.remove(&client_id);
          }
          Event::Message(client_id, message) => {
            let responder = clients.get(&client_id).unwrap();

            match message {
              Message::Text(text) => {
                logger::log(format!("Flooed frontend sent message: {}", text));

                // See if there is an associated command
                let command: Command = match serde_json::from_str(&text) {
                  Ok(c) => c,
                  Err(e) => {
                    logger::log(format!("Error parsing command: {}", e));
                    responder.send(Message::Text("Error parsing command".to_string()));
                    continue;
                  }
                };

                if commands.contains_key(&command.command) {
                  let callback = commands.get(&command.command).unwrap();
                  let result = callback(command.data.clone()).unwrap_or_default();
                  let str: Value = miniserde::json::from_str(&result).unwrap_or_else(|_| {
                    Value::String(result)
                  });

                  let resp_command = Command {
                    command: "response".to_string(),
                    data: Some(serde_json::to_value(result).unwrap()),
                    id: command.id,
                  };

                  responder.send(Message::Text(serde_json::to_string(&resp_command).unwrap()));
                } else {
                  logger::log(format!("Command not found: {}", command.command));
                  responder.send(Message::Text(format!("Command not found: {}", command.command)));
                }
              }
              Message::Binary(data) => {
                logger::log(format!("Flooed frontend sent binary data: {:?}", data));
                responder.send(Message::Binary(data));
              }
            }
          }
        }
      }
    });
  }

  pub fn register_command(&mut self, command: &str, callback: CallbackFn) {
    self.commands.insert(command.to_string(), callback);
  }
}
