use core::fmt;

/// The log level of a log message. This is used to indicate the severity and type of the message.
///
/// They can be used to format logs or filter log messages to only show certain types of messages in
/// the handler systems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Command,
    Chat,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    /// Returns the string representation of the log level, as it should appear in the log.
    /// For example, `LogLevel::Info.as_str()` will return `"INFO"`.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        return match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Command => "COMMAND",
            LogLevel::Chat => "CHAT",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        };
    }

    /// Returns the ANSI color code prefix for the log level, which can be used to color the log
    /// message when writing it to the console.
    /// For example, `LogLevel::Error.color_code()` will return `"\x1b[31m"`, which is the color
    /// code for red.
    #[must_use]
    pub const fn color_code(&self) -> &'static str {
        return match self {
            // TODO: base color code on terminal background color, e.g. use bright colors for
            // dark backgrounds and dark colors for light backgrounds
            LogLevel::Trace => "\x1b[36m",     // bright black
            LogLevel::Debug => "\x1b[32m",     // bright cyan
            LogLevel::Info => "\x1b[94m",      // bright
            LogLevel::Command => "\x1b[95m",   // bright magenta
            LogLevel::Chat => "\x1b[97m",      // magenta
            LogLevel::Warn => "\x1b[93m",      // yellow
            LogLevel::Error => "\x1b[91;1m",   // bright red and bold
            LogLevel::Fatal => "\x1b[31;1;5m", // red, bold and blinking
        };
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
        assert_eq!(LogLevel::Fatal.as_str(), "FATAL");
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
        assert_eq!(LogLevel::Fatal.to_string(), "FATAL");
    }
}
