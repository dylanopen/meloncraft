//! Module for struct [`Identifier`].

/// Identifiers are essentially a wrapper around a `String`.
///
/// Identifiers represent a unique identifier for something in the game, such as a block, item, entity,
/// painting variant, etc. They're mainly sourced from the **registries**.
///
/// ## Fields
/// 0. The `String` inside the `Identifier` struct is the actual identifier string, which follows
///    the namespacing format of `namespace:path` explained below.
///
/// ## Namespacing
/// Identifiers are namespaced, which means they are made up of two parts:
/// - The **namespace**: This is the part before the colon (`:`) in an identifier. It usually
///   represents the mod, plugin or data pack that added the thing being identified. This will
///   be `minecraft` for vanilla items. For example, in the identifier `minecraft:stone`, the
///   namespace is `minecraft`.
/// - The **path**: This is the part after the colon (`:`) in an identifier. It usually represents
///   the specific thing being identified. For example, in the identifier `minecraft:stone`, the
///   path is `stone`.
///
/// ## Packet usage
/// Identifiers are used all the time in packets, mainly during the `configuration` and `play` connection
/// states. You can use them like you would use a string, just wrap the value in the `Identifier`
/// struct. For example, to represent the identifier `minecraft:stone`, you would use
/// `Identifier("minecraft:stone".to_string())`.
///
/// ## Constraints
/// - The string inside an `Identifier` must be **less than or equal to** `32767` characters in
///   length.
/// - The string probably needs to be ASCII encoding.
/// - It should follow the format of `namespace:path`, where `namespace` and `path` are separated by
///   a colon (`:`).
#[derive(Clone, Debug)]
pub struct Identifier(pub String);
