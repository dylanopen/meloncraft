use meloncraft_text::NbtText;

#[derive(Debug, Clone)]
#[repr(i32)]
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
        match value {
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
        }
    }
}

impl From<BuiltinPauseMenuLabel> for i32 {
    fn from(label: BuiltinPauseMenuLabel) -> Self {
        match label {
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
        }
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
