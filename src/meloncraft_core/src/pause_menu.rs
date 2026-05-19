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
/// ```rust
/// use meloncraft_text::NbtText;
/// use meloncraft_text::pause_menu::{BuiltinPauseMenuLabel, PauseMenuLabel, PauseMenuLink};
/// let link = PauseMenuLink {
///     label: PauseMenuLabel::Builtin(BuiltinPauseMenuLabel::Support),
///     url: "https://support.myserver.net/".to_string(),
/// };
/// ```
/// See the [`PauseMenuLink`] documentation for more information on how to build a link.
#[derive(Debug, Clone)]
pub enum BuiltinPauseMenuLabel {

    BugReport,

    CommunityGuidelines,

    Support,

    Status,
    
    Feedback,

    Community,

    Website,

    Forums,

    News,

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

#[derive(Debug, Clone)]
pub struct CustomPauseMenuLabel(pub NbtText);

#[derive(Debug, Clone)]
pub enum PauseMenuLabel {
    Builtin(BuiltinPauseMenuLabel),
    Custom(CustomPauseMenuLabel),
}

#[derive(Debug, Clone)]
pub struct PauseMenuLink {
    pub label: PauseMenuLabel,
    pub url: String,
}
