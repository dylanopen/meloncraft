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
    ///
    /// ## Fields
    /// 0. [`BuiltinPauseMenuLabel`]: the built-in label variant to use as the label of a link.
    Builtin(BuiltinPauseMenuLabel),

    /// Represents a custom label to use as the label of a link in the pause menu.
    /// These are labels that you can customize and send to the client, and the client will display
    /// the text you sent as the label of the link in the pause menu. To construct one, you need to
    /// use the [`CustomPauseMenuLabel`] struct, which just wraps an [`NbtText`], representing the
    /// text of the label.
    ///
    /// See the [`CustomPauseMenuLabel`] documentation for more information.
    ///
    /// ## Fields
    /// 0. [`CustomPauseMenuLabel`]: the custom label to use as the label of a link.
    Custom(CustomPauseMenuLabel),
}

/// Represents the link text and URL to send in the client's pause menu.
///
/// This is the highest-level struct for representing a link: it stores:
/// - The label of the link, which can be either a built-in label (see [`BuiltinPauseMenuLabel`]) or
///   a custom label (see [`CustomPauseMenuLabel`]), wrapped in the [`PauseMenuLabel`] enum.
/// - The URL of the link, which is just a string representing the URL that the client will open when
///   they click on the link in the pause menu.
///
/// ## Packet usage
/// This struct is used in two main packets:
/// - `ClientboundConfigurationServerLinks`: during the configuration stage, you can send a list of
///   [`PauseMenuLink`]s to the client to show them in the pause menu.
/// - `ClientboundPlayServerLinks`: during the play stage, you can also send a list of
///   [`PauseMenuLink`]s to the client to show in the pause menu. This is useful if you want to change
///   the links shown in the pause menu after the configuration stage, or if you want to show
///   different links in the pause menu for different players.
///
/// ## Example
/// ```rust
/// use meloncraft_core::pause_menu::{PauseMenuLink, PauseMenuLabel, BuiltinPauseMenuLabel};
/// let link = PauseMenuLink {
///     label: PauseMenuLabel::Builtin(BuiltinPauseMenuLabel::Support),
///     url: "https://support.minecraft.net/".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct PauseMenuLink {

    /// The label of the link, which can be either a built-in label (see [`BuiltinPauseMenuLabel`])
    /// or a custom label (see [`CustomPauseMenuLabel`]), wrapped in the [`PauseMenuLabel`] enum.
    /// 
    /// This is the text displayed to the client in the pause menu. It's the part that the client
    /// clicks on to open the URL, so it should be descriptive of the URL it's linking to.
    pub label: PauseMenuLabel,

    /// The URL of the link, which is just a string representing the URL that the client will open
    /// when they click on the link in the pause menu.
    ///
    /// ## Constraints
    /// - The URL should be a valid URL that the client can open. It should probably start with
    ///   `http://` or `https://`.
    pub url: String,
}
