use core::fmt;

/// The log level of a log message. This is used to indicate the severity and type of the message.
///
/// They can be used to format logs or filter log messages to only show certain types of messages in
/// the handler systems.
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

    /// Returns the string representation of the log level, as it should appear in the log.
    /// For example, `LogLevel::Info.as_str()` will return `"INFO"`.
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

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level_as_str() {
        assert_eq!(LogLevel::Trace.as_str(), "TRACE");
        assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Command.as_str(), "COMMAND");
        assert_eq!(LogLevel::Chat.as_str(), "CHAT");
        assert_eq!(LogLevel::Warn.as_str(), "WARN");
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
    }

    #[test]
    fn log_level_display() {
        assert_eq!(LogLevel::Trace.to_string(), "TRACE");
        assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Command.to_string(), "COMMAND");
        assert_eq!(LogLevel::Chat.to_string(), "CHAT");
        assert_eq!(LogLevel::Warn.to_string(), "WARN");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }
}

