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
