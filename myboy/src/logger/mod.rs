#[derive(Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum AnsiProperty {
    Color(Color),
    Bold,
    Underline,
    Blink,
    Reverse,
    Disabled,
}

fn properties(message: &str, properties: &[AnsiProperty]) -> String {
    let mut property_str = String::new();
    properties.into_iter().for_each(|property| {
        let prop_n = match property {
            AnsiProperty::Color(Color::Black) => 30,
            AnsiProperty::Color(Color::Red) => 31,
            AnsiProperty::Color(Color::Green) => 32,
            AnsiProperty::Color(Color::Yellow) => 33,
            AnsiProperty::Color(Color::Blue) => 34,
            AnsiProperty::Color(Color::Magenta) => 35,
            AnsiProperty::Color(Color::Cyan) => 36,
            AnsiProperty::Color(Color::White) => 37,
            AnsiProperty::Bold => 1,
            AnsiProperty::Underline => 4,
            AnsiProperty::Blink => 5,
            AnsiProperty::Reverse => 7,
            AnsiProperty::Disabled => 8,
        };
        property_str.push_str(&format!(";{}", prop_n));
    });
    format!("\x1b[{}m{}\x1b[0m", property_str, message)
}

pub fn log(level: LogLevel, message: &str) {
    let level_str = match level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
    };
    let level_style = get_level_style(level);
    println!(
        "[{:^8}]     {}",
        &properties(level_str, &level_style),
        message
    );
}

fn get_level_style(level: LogLevel) -> [AnsiProperty; 1] {
    return match level {
        LogLevel::Debug => [AnsiProperty::Color(Color::Cyan)],
        LogLevel::Info => [AnsiProperty::Color(Color::Green)],
        LogLevel::Warn => [AnsiProperty::Color(Color::Yellow)],
        LogLevel::Error => [AnsiProperty::Color(Color::Red)],
    };
}

pub fn log_info(message: &str) {
    log(LogLevel::Info, message);
}

pub fn log_debug(message: &str) {
    log(LogLevel::Debug, message);
}

pub fn log_warn(message: &str) {
    log(LogLevel::Warn, message);
}

pub fn log_error(message: &str) {
    log(LogLevel::Error, message);
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)*) => {
        logger::log_info(&format!($($arg)*).as_str());
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! debug {
    ($($arg:tt)*) => {
        logger::log_debug(&format!($($arg)*).as_str());
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! warning {
    ($($arg:tt)*) => {
        logger::log_warning(&format!($($arg)*).as_str());
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! error {
    ($($arg:tt)*) => {
        logger::log_error(&format!($($arg)*).as_str());
    }
}

pub use {debug, error, info, warning};
