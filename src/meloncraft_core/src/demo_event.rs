//! Module for [`DemoEventType`] enum.

/// The type of a demo event.
///
/// Demo events are the notifications that appear in the top-right of the client's screen, which
/// explain basic Minecraft mechanics, such as movement controls, jump control, inventory control,
/// etc.
///
/// Used in the `GameEvent` packet.
#[derive(Debug, Clone, Copy)]
pub enum DemoEventType {

    /// Notification to welcome the client to the demo screen.
    WelcomeToDemoScreen,

    /// Demo screen notification explaining the basic movement controls of the game.
    MovementControls,

    /// Demo screen notification explaining how the client should jump.
    JumpControl,

    /// Demo screen notification explaining how the client can open their inventory (with e).
    InventoryControl,

    /// Demo screen notification to inform the client that the demo has finished.
    /// In addition to ending the demo, this also shows the client how to take a screenshot.
    DemoOver, 
}

impl From<DemoEventType> for u8 {
    fn from(value: DemoEventType) -> Self {
        return match value {
            DemoEventType::WelcomeToDemoScreen => 0,
            DemoEventType::MovementControls => 101,
            DemoEventType::JumpControl => 102,
            DemoEventType::InventoryControl => 103,
            DemoEventType::DemoOver => 104,
        };
    }
}

impl TryFrom<u8> for DemoEventType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(DemoEventType::WelcomeToDemoScreen),
            101 => Ok(DemoEventType::MovementControls),
            102 => Ok(DemoEventType::JumpControl),
            103 => Ok(DemoEventType::InventoryControl),
            104 => Ok(DemoEventType::DemoOver),
            _ => Err(()),
        };
    }
}


impl From<DemoEventType> for f32 {
    fn from(demo_event_type: DemoEventType) -> Self {
        return u8::from(demo_event_type).into();
    }
}

impl TryFrom<f32> for DemoEventType {
    type Error = ();
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        return match value {
            0.0 => Ok(DemoEventType::WelcomeToDemoScreen),
            101.0 => Ok(DemoEventType::MovementControls),
            102.0 => Ok(DemoEventType::JumpControl),
            103.0 => Ok(DemoEventType::InventoryControl),
            104.0 => Ok(DemoEventType::DemoOver),
            _ => Err(()),
        };
    }
}
