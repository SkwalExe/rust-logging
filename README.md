# Rust logging ⛔

![](images/1.png)

💬 A couple of functions to make logging in Rust easier.

# Installation 📦

Just add the code of [code.rs](code.rs) to your project.

You can copy/paste the code to your project [here](#copypaste-).

# Usage 📝

This code provides the following functions:

- 🟡 **warn()**: Prints a warning message.
- 🔴 **error()**: Prints an error message.
- 🔵 **info()**: Prints an information message.
- 🟢 **success()**: Prints a success message.
- ℹ️ Add a `f` before the function name to print the message to the log file

Each function takes a single parameter, which is the message to be printed.

```rust
error("a command failed : hello"); 
info("executing command : hello");
warn("a command is about to fail : hello");
success("a command succeeded : hello");
```

## Highlighting 🔍

By default, the text after the colon is highlighted.

![](images/1.png)

This can be disabled by setting the `HIGHLIGHT` constant to `false`.

```
const HIGHLIGHT: bool = false;
```

![](images/2.png)

## Icon connector 🔗

By default, the icon and the message are separated by an arrow `->`

![](images/2.png)

You can change this by setting the `ICON_CONNECTOR` constant to something else.

```
const ICON_CONNECTOR: &str = "🔗";
```

![](images/3.png)

## Icons 🔍

By default, the following icons are used:

| Icon  | Function  |
| ----- | --------- |
| [ x ] | error()   |
| [ i ] | info()    |
| [ v ] | success() |
| [ ! ] | warn()    |

You can change this by setting the following constants:

```rust
const ERROR_ICON: &str = "[ ⛔ ]";
const INFO_ICON: &str = "[ ℹ️ ]";
const SUCCESS_ICON: &str = "[ ✅ ]";
const WARNING_ICON: &str = "[ ⚠️ ]";
```

![](images/4.png)

## Log file 📄

You can specify the log file path to write the messages to with the `LOG_FILE` constant.

Use the `f` prefix before the function name to print the message to the log file.

```rust
const LOG_FILE: &str = "myProgram.log";

// -----------------------------

fwarning("This is a : warning message");
fsuccess("This is a : success message");
finfo("This is an : information message");
ferror("This is an : error message");
```

![](images/5.png)

## also logging to terminal when logging to file 📄

You can set environment variable `LOG` to `print` to also log to the terminal when logging to a file.

```bash
LOG=print [program]

#########
# PROGRAM SOURCE CODE #
# ferror("This is an : error message");
#########

[Terminal output] : 

This is an : error message

#########
# LOGFILE CONTENT #
# This is an : error message
#########
```

# Copy/paste 📋

```rust
/// PARAMETERS:

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
```

# final

If you have any problem, don't hesitate to open an issue

# contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

<a href="https://github.com/SkwalExe#ukraine"><img src="https://raw.githubusercontent.com/SkwalExe/SkwalExe/main/ukraine.jpg" width="100%" height="15px" /></a>