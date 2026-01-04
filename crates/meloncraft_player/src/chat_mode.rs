use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy)]
#[repr(i32)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

impl TryFrom<i32> for ChatMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(()),
        }
    }
}
