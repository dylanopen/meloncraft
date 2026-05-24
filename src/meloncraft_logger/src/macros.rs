#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)+) => {
        $crate::log::Log {
            level: $level,
            source: format!("{}:{}", file!(), line!()),
            message: format!($($arg)+),
        }
    };
}

#[macro_export]
macro_rules! tracelog {
    ($($arg:tt)*) => {
        $crate::macros::log_helper!($crate::level::LogLevel::Trace, $($arg)*)
    }
}

#[macro_export]
macro_rules! debuglog {
    ($($arg:tt)*) => {
        $crate::macros::log_helper!(LogLevel::Debug, $($arg)*)
    }
}

#[macro_export]
macro_rules! infolog {
    ($($arg:tt)*) => {
        $crate::macros::log_helper!(LogLevel::Info, $($arg)*)
    }
}

#[macro_export]
macro_rules! commandlog {
    ($($arg:tt)*) => {
        $crate::macros::log_helper!(LogLevel::Command, $($arg)*)
    }
}

#[macro_export]
macro_rules! chatlog {
    ($($arg:tt)*) => {
        $crate::macros::log_helper!(LogLevel::Chat, $($arg)*)
    }
}

#[macro_export]
macro_rules! warnlog {
    ($($arg:tt)*) => {
        $crate::macros::log_helper!(LogLevel::Warn, $($arg)*)
    }
}

#[macro_export]
macro_rules! errorlog {
    ($($arg:tt)*) => {
        $crate::macros::log_helper!(LogLevel::Error, $($arg)*)
    }
}

