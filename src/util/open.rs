use std::{path::PathBuf, process::Command};

pub fn open_folder(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(target_os = "windows")]
  Command::new("explorer").arg(path).spawn()?;

  #[cfg(target_os = "macos")]
  Command::new("open").arg(path).spawn()?;

  #[cfg(target_os = "linux")]
  Command::new("xdg-open").arg(path).spawn()?;

  Ok(())
}

pub fn open_scheme(scheme: String) -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(target_os = "windows")]
  Command::new("start").arg(scheme).spawn()?;

  #[cfg(target_os = "macos")]
  Command::new("open").arg(scheme).spawn()?;

  #[cfg(target_os = "linux")]
  Command::new("xdg-open").arg(scheme).spawn()?;

  Ok(())
}