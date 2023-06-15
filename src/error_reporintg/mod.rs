use log::{error, info, warn};

pub fn error(line: usize, message: &str) {
    report(line, message, 0);
}

pub fn warn(line: usize, message: &str) {
    report(line, message, 1);
}

pub fn info(message: &str) {
    report(0, message, 2);
}

pub fn report(line: usize, message: &str, level: i32) {
    if level == 0 {
        error!("[line {}] Error: {}", line, message);
    }

    if level == 1 {
        warn!("[line {}] Warning: {}", line, message);
    }

    if level == 2 {
        info!("Info: {}", message);
    }
}
