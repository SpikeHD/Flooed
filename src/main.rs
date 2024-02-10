#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use webui_rs::webui::{wait, Window, WebUIBrowser};

fn main() {
  let win = Window::new();

  win.show_browser(r#"
  <html>
    <script src="/webui.js"></script>
    <iframe src="https://discord.com/app" style="width: 100%; height: 100%; border: none;"></iframe>
  </html>
  "#, WebUIBrowser::ChromiumBased);

  wait();
}
