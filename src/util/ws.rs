use std::{collections::HashMap, sync::Arc};
use serde::{Serialize, Deserialize};
use simple_websockets::{self, Event, Message, Responder};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Command {
  command: String,
  data: Option<Value>
}

pub struct WsConnector {
  ws: Arc<simple_websockets::EventHub>,
  commands: HashMap<String, fn(Option<Value>)>
}

impl WsConnector {
  pub fn new() -> WsConnector {
    WsConnector {
      ws: Arc::new(simple_websockets::launch(10102).expect("Could not launch WebSocket server. Is Flooed already running?")),
      commands: HashMap::new()
    }
  }

  /**
   * Start the WebSocket server. Commands cannot be registered after this is called.
   */
  pub fn start(&self) {
    let ws = self.ws.clone();
    let commands = self.commands.clone();

    std::thread::spawn(move || {
      let mut clients: HashMap<u64, Responder> = HashMap::new();

      loop {
        match ws.poll_event() {
          Event::Connect(client_id, responder) => {
            println!("Flooed frontend connected: {}", client_id);
            
            // Only insert if there isn't already an active client
            if clients.len() == 0 {
              clients.insert(client_id, responder);
            } else {
              println!("Flooed frontend already connected: {}", client_id);
            }
          },
          Event::Disconnect(client_id) => {
            println!("Flooed frontend disconnected: {}", client_id);
            clients.remove(&client_id);
          },
          Event::Message(client_id, message) => {
            let responder = clients.get(&client_id).unwrap();

            match message {
              Message::Text(text) => {
                println!("Flooed frontend sent message: {}", text);
                
                // See if there is an associated command
                let command: Command = match serde_json::from_str(&text) {
                  Ok(c) => c,
                  Err(e) => {
                    println!("Error parsing command: {}", e);
                    responder.send(Message::Text("Error parsing command".to_string()));
                    continue;
                  }
                };

                if commands.contains_key(&command.command) {
                  let callback = commands.get(&command.command).unwrap();
                  callback(command.data.clone());
                } else {
                  println!("Command not found: {}", command.command);
                  responder.send(Message::Text("Command not found".to_string()));
                }
              },
              Message::Binary(data) => {
                println!("Flooed frontend sent binary data: {:?}", data);
                responder.send(Message::Binary(data));
              }
            }
          }
        }
      }
    });
  }
  
  pub fn register_command(&mut self, command: &str, callback: fn(Option<Value>)) {
    self.commands.insert(command.to_string(), callback);
  }
}