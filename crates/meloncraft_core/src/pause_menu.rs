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

#[derive(Debug, Clone)]
pub struct CustomPauseMenuLabel(pub NbtText);
