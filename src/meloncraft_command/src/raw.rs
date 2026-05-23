//! Module for struct message [`RawCommand`].

use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;


/// Represents a raw command, which is a command that has not been parsed yet. It contains the name
/// of the command and its arguments as strings.
#[derive(Message)]
pub struct RawCommand {

    /// The entity that executed the command. This is usually a player, but could be any entity
    /// that can execute commands, such as a command block or a function (when they are
    /// implemented, of course).
    ///
    /// This can be used to respond to the command or to get context about who executed it.
    pub executor: Entity,

    /// The `name` is the first word of the raw command string, which is usually the command name. For
    /// example, in the command string `"/give meloncrafter diamond 64"`, the `name` would be `"give"`.
    ///
    /// The leading slash is not included in the `name`.
    pub name: String,

    /// The `args` are just built from splitting the raw command string by spaces.
    /// For example, in the command string `"/give meloncrafter diamond 64"`, the `args` would be
    /// `["meloncrafter", "diamond", "64"]`.
    pub args: Vec<String>,
}
