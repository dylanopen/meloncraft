//! Module for struct [`Log`].

use bevy::ecs::message::Message;

use crate::level::LogLevel;

/// A log message.
/// Has a level, source (file and line number) and the message itself.
/// 
/// ## Reading
/// As this is a message, is can be *read* by any systems which want to write the log to, say, a
/// file or the console.
///
/// ## Writing
/// You can write a log message like any other Bevy message, using a `MessageWriter<Log>` in a
/// system.
/// You can also use the macros in [`crate::macros`] to write log messages more easily.
#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub struct Log {

    /// The level of the log message.
    /// This can be used to filter log messages, for example, to only write error and warn messages
    /// to a file, but write all messages to the console.
    ///
    /// ## Using macros
    /// The level is determined by the macro used to write the log message, so you can just use,
    /// say, debuglog! or errorlog! to create a log message with the Debug or Error `LogLevel`.
    pub level: LogLevel,

    /// The source of the log message, which is usually the file and line number where the log message was written.
    /// This can be used to quickly find the source of a log message when reading the logs.
    ///
    /// You should write this in the form:
    /// ```text
    /// file.rs:line
    /// e.g. movement.rs:42
    /// ```
    pub source: String,

    /// The message itself, which is the content of the log message.
    /// This can be any string, and can contain any information you want to log.
    ///
    /// You can use the macros in [`crate::macros`] to write log messages more easily, which will allow you to use
    /// format strings and arguments, like this:
    /// ```rust
    /// info!("Player {} has joined the game", player_name);
    /// ```
    pub message: String,
}
