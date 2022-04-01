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

# Copy/paste 📋

```rust
// PARAMETERS:

const ICON_CONNECTOR: &str = "->";      // connector between icon and message
const ERROR_ICON: &str = "[ x ]";       // icon for errors
const INFO_ICON: &str = "[ i ]";        // icon for informations
const SUCCESS_ICON: &str = "[ v ]";     // icon for success
const WARNING_ICON: &str = "[ ! ]";     // icon for warnings
const HIGHLIGHT: bool = true;           // should the text after ":" be highlighted

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

fn warn(msg: &str) {
    let msg = String::from(msg);

    let mut msg_vec = msg.split(":").collect::<Vec<&str>>();

    println!(
        "{}{} {} {}{}{}",
        YELLOW,
        WARNING_ICON,
        ICON_CONNECTOR,
        msg_vec.remove(0),
        if msg_vec.len() > 0 {
            format!(
                ":{}{} ",
                if HIGHLIGHT {
                    format!(" {}{}", BG_YELLOW, WHITE)
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

fn success(msg: &str) {
    let msg = String::from(msg);

    let mut msg_vec = msg.split(":").collect::<Vec<&str>>();

    println!(
        "{}{} {} {}{}{}",
        GREEN,
        SUCCESS_ICON,
        ICON_CONNECTOR,
        msg_vec.remove(0),
        if msg_vec.len() > 0 {
            format!(
                ":{}{} ",
                if HIGHLIGHT {
                    format!(" {}{}", BG_GREEN, WHITE)
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

fn info(msg: &str) {
    let msg = String::from(msg);

    let mut msg_vec = msg.split(":").collect::<Vec<&str>>();

    println!(
        "{}{} {} {}{}{}",
        CYAN,
        INFO_ICON,
        ICON_CONNECTOR,
        msg_vec.remove(0),
        if msg_vec.len() > 0 {
            format!(
                ":{}{} ",
                if HIGHLIGHT {
                    format!(" {}{}", BG_CYAN, WHITE)
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

fn error(msg: &str) {
    let msg = String::from(msg);

    let mut msg_vec = msg.split(":").collect::<Vec<&str>>();

    eprintln!(
        "{}{} {} {}{}{}",
        RED,
        ERROR_ICON,
        ICON_CONNECTOR,
        msg_vec.remove(0),
        if msg_vec.len() > 0 {
            format!(
                ":{}{} ",
                if HIGHLIGHT {
                    format!(" {}{}", BG_RED, WHITE)
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
```

# final

If you have any problem, don't hesitate to open an issue

# contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

<a href="https://github.com/SkwalExe#ukraine"><img src="https://raw.githubusercontent.com/SkwalExe/SkwalExe/main/ukraine.jpg" width="100%" height="15px" /></a>