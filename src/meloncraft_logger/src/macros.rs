#[macro_export]
macro_rules! log_helper {
    ($level:expr, $($arg:tt)+) => {
        Log {
            level: $level,
            source: format!("{}:{}", file!(), line!()),
            message: format!($($arg)+),
        }
    };
}

#[macro_export]
macro_rules! tracelog {
    ($($arg:tt)*) => {
        log_helper!(LogLevel::Trace, $($arg)*)
    }
}

#[macro_export]
macro_rules! debuglog {
    ($($arg:tt)*) => {
        log_helper!(LogLevel::Debug, $($arg)*)
    }
}

#[macro_export]
macro_rules! infolog {
    ($($arg:tt)*) => {
        log_helper!(LogLevel::Info, $($arg)*)
    }
}

#[macro_export]
macro_rules! commandlog {
    ($($arg:tt)*) => {
        log_helper!(LogLevel::Command, $($arg)*)
    }
}

#[macro_export]
macro_rules! chatlog {
    ($($arg:tt)*) => {
        log_helper!(LogLevel::Chat, $($arg)*)
    }
}

#[macro_export]
macro_rules! warnlog {
    ($($arg:tt)*) => {
        log_helper!(LogLevel::Warn, $($arg)*)
    }
}

#[macro_export]
macro_rules! errorlog {
    ($($arg:tt)*) => {
        log_helper!(LogLevel::Error, $($arg)*)
    }
}

