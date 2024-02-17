#[macro_export]
macro_rules! register_command {
  ($ws:expr, $command_name:ident, $callback:expr) => {
    $ws.register_command(
      stringify!($command_name),
      std::sync::Arc::new(std::sync::Mutex::new($callback)),
    );
  };
}
