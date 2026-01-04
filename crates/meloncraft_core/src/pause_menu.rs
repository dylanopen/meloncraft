use meloncraft_text::NbtText;

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

#[derive(Debug, Clone)]
pub struct CustomPauseMenuLabel(pub NbtText);

#[derive(Debug, Clone)]
pub enum PauseMenuLabel {
    Builtin(BuiltinPauseMenuLabel),
    Custom(CustomPauseMenuLabel),
}
