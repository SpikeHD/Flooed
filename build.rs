fn main() {
  // WebUI static lib
  #[cfg(not(target_os = "windows"))]
  {
      println!("cargo:rustc-link-search=native=./");
  }
}
