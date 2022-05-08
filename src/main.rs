use rust_logging::*;

fn main() {
  println!("Let try using the default settings");
  let logger_default = Logger::new();

  logger_default.error("I am an : error message");
  logger_default.info("I am an : info message");
  logger_default.warn("I am a : warning message");
  logger_default.success("I am a : success message");

  println!("\nLet try using custom settings");
  let custom_opts = LoggerOptions {
    icon_connector: "=>".to_string(),
    error_icon: "🔥".to_string(),
    info_icon: "💡".to_string(),
    warning_icon: "⚠️ ".to_string(),
    success_icon: "✅".to_string(),
    highlight: false,
    ..Default::default()
  };

  let logger_custom = custom_opts.get_logger();

  logger_custom.error("I am an : error message");
  logger_custom.info("I am an : info message");
  logger_custom.warn("I am a : warning message");
  logger_custom.success("I am a : success message");
}
