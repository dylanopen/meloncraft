#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)+) => {
        $crate::buffer::LOG_BUFFER.lock().unwrap().push($crate::log::Log {
            level: $level,
            source: format!("{}:{}", file!(), line!()),
            message: format!($($arg)+),
        });
    };
}

#[macro_export]
macro_rules! tracelog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Trace, $($arg)+);
    };
}

#[macro_export]
macro_rules! debuglog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Debug, $($arg)+);
    };
}

#[macro_export]
macro_rules! commandlog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Command, $($arg)+);
    };
}

#[macro_export]
macro_rules! chatlog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Chat, $($arg)+);
    };
}

#[macro_export]
macro_rules! infolog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Info, $($arg)+);
    };
}

#[macro_export]
macro_rules! warnlog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Warn, $($arg)+);
    };
}

#[macro_export]
macro_rules! errorlog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Error, $($arg)+);
    };
}

#[macro_export]
macro_rules! fatallog {
    ($($arg:tt)+) => {
        $crate::log!($crate::level::LogLevel::Fatal, $($arg)+);
    };
}
