use console::Style;
use chrono::Local;

fn log_message(level: &str, color: Style, msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("{} [{}] {}", color.apply_to(level), timestamp, msg);
}

pub fn error(msg: &str) {
    let style = Style::new().red().bold();
    log_message("ERROR", style, msg);
}

pub fn warn(msg: &str) {
    let style = Style::new().yellow().bold();
    log_message("WARN", style, msg);
}

pub fn info(msg: &str) {
    let style = Style::new().blue();
    log_message("INFO", style, msg);
}

pub fn debug(msg: &str) {
    let style = Style::new().purple();
    log_message("DEBUG", style, msg);
}

pub fn success(msg: &str) {
    let style = Style::new().green().bold();
    log_message("SUCCESS", style, msg);
}


