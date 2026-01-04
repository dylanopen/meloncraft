use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

impl TryFrom<i32> for MainHand {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => Err(()),
        }
    }
}
