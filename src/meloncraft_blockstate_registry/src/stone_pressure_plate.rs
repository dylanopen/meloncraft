use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonePressurePlate {
    pub powered: bool,
}

impl BlockState for StonePressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == true {
            return 6594;
        }
        if self.r#powered == false {
            return 6595;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6594 {
            return Some(StonePressurePlate { r#powered: true });
        }
        if state_id == 6595 {
            return Some(StonePressurePlate { r#powered: false });
        }
        return None;
    }
}
