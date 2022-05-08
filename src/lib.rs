use isatty::{stderr_isatty, stdout_isatty};
use std::default::Default;
use std::fmt::Display;

pub struct Colors {
    red: String,
    green: String,
    yellow: String,
    cyan: String,
    white: String,
    reset: String,
    bg_red: String,
    bg_cyan: String,
    bg_green: String,
    bg_yellow: String,
}

impl Colors {
    pub fn new() -> Colors {
        if stderr_isatty() && stdout_isatty() {
            Colors {
                red: "\x1b[91m".to_string(),
                green: "\x1b[92m".to_string(),
                yellow: "\x1b[93m".to_string(),
                cyan: "\x1b[96m".to_string(),
                white: "\x1b[97m".to_string(),
                reset: "\x1b[0m".to_string(),
                bg_red: "\x1b[41m".to_string(),
                bg_cyan: "\x1b[46m".to_string(),
                bg_green: "\x1b[42m".to_string(),
                bg_yellow: "\x1b[43m".to_string(),
            }
        } else {
            Colors {
                red: "".to_string(),
                green: "".to_string(),
                yellow: "".to_string(),
                cyan: "".to_string(),
                white: "".to_string(),
                reset: "".to_string(),
                bg_red: "".to_string(),
                bg_cyan: "".to_string(),
                bg_green: "".to_string(),
                bg_yellow: "".to_string(),
            }
        }
    }
}

enum MessageTypes {
    ERROR,
    INFO,
    SUCCESS,
    WARNING,
}

enum LogTypes {
    Terminal,
    File,
}

pub struct LoggerOptions {
    pub icon_connector: String,
    pub error_icon: String,
    pub info_icon: String,
    pub warning_icon: String,
    pub success_icon: String,
    pub highlight: bool,
    pub log_file: String,
    pub colors: Colors,
}

impl Default for LoggerOptions {
    fn default() -> LoggerOptions {
        LoggerOptions {
            icon_connector: "->".to_string(),
            error_icon: "[ x ]".to_string(),
            info_icon: "[ i ]".to_string(),
            warning_icon: "[ ! ]".to_string(),
            success_icon: "[ v ]".to_string(),
            highlight: true,
            log_file: "logs.log".to_string(),
            colors: Colors::new(),
        }
    }
}

impl LoggerOptions {
    fn new() -> Self {
        Self {
            icon_connector: "->".to_string(),
            error_icon: "[ x ]".to_string(),
            info_icon: "[ i ]".to_string(),
            warning_icon: "[ ! ]".to_string(),
            success_icon: "[ v ]".to_string(),
            highlight: true,
            log_file: "logs.log".to_string(),
            colors: Colors::new(),
        }
    }

    pub fn get_logger(self) -> Logger {
        let mut logger = Logger::new();
        logger.options = self;
        logger
    }
}

impl Display for LoggerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Icon connector : {}\nError Icon : {}\nInfo Icon : {}\nWarning Icon : {}\nSuccess Icon : {}\nHighlight : {}\nLog File : {}", self.icon_connector, self.error_icon, self.info_icon, self.warning_icon, self.success_icon, self.highlight, self.log_file)
    }
}

pub struct Logger {
    options: LoggerOptions,
}

impl Logger {
    fn rust_logging_print(&self, msg: &str, msg_type: MessageTypes, log_type: LogTypes) {
        use std::fs::OpenOptions;
        use std::io::Write;
        let msg = String::from(msg);
        let icon = match msg_type {
            MessageTypes::ERROR => self.options.error_icon.clone(),
            MessageTypes::INFO => self.options.info_icon.clone(),
            MessageTypes::SUCCESS => self.options.success_icon.clone(),
            MessageTypes::WARNING => self.options.warning_icon.clone(),
        };
        match log_type {
            LogTypes::File => {
                let mut file = match OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(self.options.log_file.clone())
                {
                    Ok(file) => file,
                    Err(e) => {
                        println!(
                            "{}Cannot open log file : {}{}",
                            self.options.colors.red, e, self.options.colors.reset
                        );
                        return;
                    }
                };
                match file.write_all(
                    format!("{} {} {}\n", icon, self.options.icon_connector, msg).as_bytes(),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "{}Cannot write to log file : {}{}",
                            self.options.colors.red, e, self.options.colors.reset
                        );
                        return;
                    }
                };
                let log_env = match std::env::var("LOG") {
                    Ok(log_env) => log_env,
                    Err(_) => "".to_string(),
                };
                if log_env == "print" {
                    println!("{}", msg);
                }
            }
            LogTypes::Terminal => {
                let color = match msg_type {
                    MessageTypes::ERROR => self.options.colors.red.clone(),
                    MessageTypes::INFO => self.options.colors.cyan.clone(),
                    MessageTypes::SUCCESS => self.options.colors.green.clone(),
                    MessageTypes::WARNING => self.options.colors.yellow.clone(),
                };
                let bg_color = match msg_type {
                    MessageTypes::ERROR => self.options.colors.bg_red.clone(),
                    MessageTypes::INFO => self.options.colors.bg_cyan.clone(),
                    MessageTypes::SUCCESS => self.options.colors.bg_green.clone(),
                    MessageTypes::WARNING => self.options.colors.bg_yellow.clone(),
                };
                let mut msg_vec = msg.split(":").collect::<Vec<&str>>();
                println!(
                    "{}{} {} {}{}{}",
                    color,
                    icon,
                    self.options.icon_connector,
                    msg_vec.remove(0),
                    if msg_vec.len() > 0 {
                        format!(
                            ":{}{} ",
                            if self.options.highlight {
                                format!(" {}{}", bg_color, self.options.colors.white.clone())
                            } else {
                                "".to_string()
                            },
                            msg_vec.join(":")
                        )
                    } else {
                        "".to_string()
                    },
                    self.options.colors.reset
                );
            }
        }
    }
    pub fn new() -> Self {
        Self {
            options: LoggerOptions::new(),
        }
    }

    pub fn new_from_opts(options: LoggerOptions) -> Self {
        Self { options: options }
    }

    pub fn warn<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::WARNING, LogTypes::Terminal);
    }
    pub fn success<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::SUCCESS, LogTypes::Terminal);
    }
    pub fn info<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::INFO, LogTypes::Terminal);
    }
    pub fn error<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::ERROR, LogTypes::Terminal);
    }
    pub fn fwarning<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::WARNING, LogTypes::File);
    }
    pub fn fsuccess<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::SUCCESS, LogTypes::File);
    }
    pub fn finfo<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::INFO, LogTypes::File);
    }
    pub fn ferror<S: AsRef<str>>(&self, msg: S) {
        self.rust_logging_print(msg.as_ref(), MessageTypes::ERROR, LogTypes::File);
    }
}
