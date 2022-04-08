// PARAMETERS:

const ICON_CONNECTOR: &str = "->";      // connector between icon and message
const ERROR_ICON: &str = "[ x ]";       // icon for errors
const INFO_ICON: &str = "[ i ]";        // icon for informations
const SUCCESS_ICON: &str = "[ v ]";     // icon for success
const WARNING_ICON: &str = "[ ! ]";     // icon for warnings
const HIGHLIGHT: bool = true;           // should the text after ":" be highlighted
const LOG_FILE: &str = "myProgram.log";       // file to log to

// ---------------

const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";
const BG_CYAN: &str = "\x1b[46m";
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
use std::fs::OpenOptions;
use std::io::Write;
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
fn rust_logging_print(msg: &str, msg_type: MessageTypes, log_type: LogTypes) {
    let msg = String::from(msg);
    let icon = match msg_type {
        MessageTypes::ERROR => ERROR_ICON,
        MessageTypes::INFO => INFO_ICON,
        MessageTypes::SUCCESS => SUCCESS_ICON,
        MessageTypes::WARNING => WARNING_ICON,
    };
    match log_type {
        LogTypes::File => {
            let mut file = match OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(LOG_FILE) {
                    Ok(file) => file,
                    Err(e) => {
                        println!("{}Cannot open log file : {}{}", RED, e, RESET);
                        return;
                    }
                };
            match file.write_all(format!("{} {} {}\n", icon,  ICON_CONNECTOR, msg).as_bytes()) {
                Ok(_) => {},
                Err(e) => {
                    println!("{}Cannot write to log file : {}{}", RED, e, RESET);
                    return;
                }
            };

            // get env var LOG
            let log_env = match std::env::var("LOG") {
                Ok(log_env) => log_env,
                Err(_) => "".to_string(),
            };

            // if LOG is set to print then print to the terminal
            if log_env == "print" {
                println!("{}", msg);
            }
        },
        LogTypes::Terminal => {
            let color = match msg_type {
                MessageTypes::ERROR => RED,
                MessageTypes::INFO => CYAN,
                MessageTypes::SUCCESS => GREEN,
                MessageTypes::WARNING => YELLOW,
            };
            let bg_color = match msg_type {
                MessageTypes::ERROR => BG_RED,
                MessageTypes::INFO => BG_CYAN,
                MessageTypes::SUCCESS => BG_GREEN,
                MessageTypes::WARNING => BG_YELLOW,
            };
            let mut msg_vec = msg.split(":").collect::<Vec<&str>>();
            println!(
                "{}{} {} {}{}{}",
                color,
                icon,
                ICON_CONNECTOR,
                msg_vec.remove(0),
                if msg_vec.len() > 0 {
                    format!(
                        ":{}{} ",
                        if HIGHLIGHT {
                            format!(" {}{}", bg_color, WHITE)
                        } else {
                            "".to_string()
                        },
                        msg_vec.join(":")
                    )
                } else {
                    "".to_string()
                },
                RESET
            );
        }
    }
}
fn warn(msg: &str) {
    rust_logging_print(msg, MessageTypes::WARNING, LogTypes::Terminal);
}
fn success(msg: &str) {
    rust_logging_print(msg, MessageTypes::SUCCESS, LogTypes::Terminal);
}
fn info(msg: &str) {
    rust_logging_print(msg, MessageTypes::INFO, LogTypes::Terminal);
}
fn error(msg: &str) {
    rust_logging_print(msg, MessageTypes::ERROR, LogTypes::Terminal);
}
fn fwarning(msg: &str) {
    rust_logging_print(msg, MessageTypes::WARNING, LogTypes::File);
}
fn fsuccess(msg: &str) {
    rust_logging_print(msg, MessageTypes::SUCCESS, LogTypes::File);
}
fn finfo(msg: &str) {
    rust_logging_print(msg, MessageTypes::INFO, LogTypes::File);
}
fn ferror(msg: &str) {
    rust_logging_print(msg, MessageTypes::ERROR, LogTypes::File);
}
