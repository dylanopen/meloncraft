#[derive(Debug, Clone)]
pub enum DemoEventType {
    WelcomeToDemoScreen,
    MovementControls,
    JumpControl,
    InventoryControl,
    DemoOver, // also shows how to take a screenshot
}

impl From<DemoEventType> for u8 {
    fn from(value: DemoEventType) -> Self {
        match value {
            DemoEventType::WelcomeToDemoScreen => 0,
            DemoEventType::MovementControls => 101,
            DemoEventType::JumpControl => 102,
            DemoEventType::InventoryControl => 103,
            DemoEventType::DemoOver => 104,
        }
    }
}

impl TryFrom<u8> for DemoEventType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DemoEventType::WelcomeToDemoScreen),
            101 => Ok(DemoEventType::MovementControls),
            102 => Ok(DemoEventType::JumpControl),
            103 => Ok(DemoEventType::InventoryControl),
            104 => Ok(DemoEventType::DemoOver),
            _ => Err(()),
        }
    }
}

