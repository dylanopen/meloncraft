use core::fmt;

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Command,
    Chat,
    Warn,
    Error,
}

impl LogLevel {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => return "TRACE",
            LogLevel::Debug => return "DEBUG",
            LogLevel::Info => return "INFO",
            LogLevel::Command => return "COMMAND",
            LogLevel::Chat => return "CHAT",
            LogLevel::Warn => return "WARN",
            LogLevel::Error => return "ERROR",
        }
    }
}

