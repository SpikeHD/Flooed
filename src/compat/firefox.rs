use webui_rs::webui::bindgen::*;

pub unsafe fn move_firefox_extension() {
  // Take the extension file (should be beside the binary)
  let ext = std::env::current_dir().unwrap().join("ext").join("mv2.xpi");
  // get the firefox profile path
}