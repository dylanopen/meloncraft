use meloncraft_text::NbtText;

/// Represents a *default* Minecraft link label to send in the client's pause menu.
///
/// These are the labels that are already built into the Minecraft client, and you can use them
/// without needing to worry about the text formatting or length constraints, since the client
/// already knows about them. They have an ID, which are documented in the individual variants.
/// 
/// ## Packet usage
/// This enum just stores variants and functions for getting the ID of each default label. The enum
/// used in packets is the [`PauseMenuLink`] enum, which stores a link and a [`PauseMenuLabel`]
/// (which can be either a default label or a custom label).
/// 
/// ## Building a [`PauseMenuLink`]
/// To build a [`PauseMenuLink`] with a default label, you can use the [`PauseMenuLabel::Builtin`]
/// variant, and pass in the default label you want to use.
/// For example, to build a link with the default `Support` label, you would do:
///
/// See the [`PauseMenuLink`] documentation for more information on how to build a link.
#[derive(Debug, Clone)]
pub enum BuiltinPauseMenuLabel {

    /// A built-in [`PauseMenuLabel`] shown on connection error screens.
    /// Built-in label ID: `0`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    BugReport,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `1`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    CommunityGuidelines,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `2`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    Support,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `3`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    Status,
    
    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `4`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    Feedback,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `5`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    Community,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `6`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    Website,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `7`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    Forums,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `8`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    News,

    /// A built-in [`PauseMenuLabel`] that can be shown in the pause menu.
    /// Built-in label ID: `9`.
    /// See [`BuiltinPauseMenuLabel`] for docs on what default labels are and how to use them.
    Announcements,
}

impl TryFrom<i32> for BuiltinPauseMenuLabel {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(BuiltinPauseMenuLabel::BugReport),
            1 => Ok(BuiltinPauseMenuLabel::CommunityGuidelines),
            2 => Ok(BuiltinPauseMenuLabel::Support),
            3 => Ok(BuiltinPauseMenuLabel::Status),
            4 => Ok(BuiltinPauseMenuLabel::Feedback),
            5 => Ok(BuiltinPauseMenuLabel::Community),
            6 => Ok(BuiltinPauseMenuLabel::Website),
            7 => Ok(BuiltinPauseMenuLabel::Forums),
            8 => Ok(BuiltinPauseMenuLabel::News),
            9 => Ok(BuiltinPauseMenuLabel::Announcements),
            _ => Err(()),
        };
    }
}

impl From<BuiltinPauseMenuLabel> for i32 {
    fn from(label: BuiltinPauseMenuLabel) -> Self {
        return match label {
            BuiltinPauseMenuLabel::BugReport => 0,
            BuiltinPauseMenuLabel::CommunityGuidelines => 1,
            BuiltinPauseMenuLabel::Support => 2,
            BuiltinPauseMenuLabel::Status => 3,
            BuiltinPauseMenuLabel::Feedback => 4,
            BuiltinPauseMenuLabel::Community => 5,
            BuiltinPauseMenuLabel::Website => 6,
            BuiltinPauseMenuLabel::Forums => 7,
            BuiltinPauseMenuLabel::News => 8,
            BuiltinPauseMenuLabel::Announcements => 9,
        };
    }
}

/// Represents a *custom* Minecraft link label (the text) to send in the client's pause menu.
///
/// These are the labels that you can customize and send to the client to change the text shown on a
/// link in the pause menu.
///
/// ## Fields
/// 0. [`NbtText`]: The text shown on the link in the pause menu. This can be formatted with colors,
///    styles, etc. using [`NbtText`] formatting. See the [`NbtText`] documentation for more
///    information on how to format text.
///
/// ## Packet usage
/// This struct is used in the [`PauseMenuLabel::Custom`] variant, which is used in the
/// [`PauseMenuLink`] struct to represent a link with a custom label. See the [`PauseMenuLink`]
/// documentation for more information on how to build a link with a custom label.
#[derive(Debug, Clone)]
pub struct CustomPauseMenuLabel(pub NbtText);

/// Represents a generic link **label** for use in the client's pause menu.
///
/// This label can be either:
/// - A built-in label, see [`PauseMenuLabel::Builtin`]
/// - A custom label, see [`PauseMenuLabel::Custom`]
///
/// ## Packet usage
/// This enum is used in the [`PauseMenuLink`] struct to represent the label of a link in the pause
/// menu. See the [`PauseMenuLink`] documentation for more information on how to build a link with
/// either a built-in or custom label.
#[derive(Debug, Clone)]
pub enum PauseMenuLabel {

    /// Represents a built-in label to use as the label of a link in the pause menu.
    /// These are labels that the client already knows about, and the only thing needed to construct
    /// one is the ID (encoded in [`BuiltinPauseMenuLabel`]) variants.
    ///
    /// See the [`BuiltinPauseMenuLabel`] documentation for more information.
    Builtin(BuiltinPauseMenuLabel),

    Custom(CustomPauseMenuLabel),
}

#[derive(Debug, Clone)]
pub struct PauseMenuLink {
    pub label: PauseMenuLabel,
    pub url: String,
}
