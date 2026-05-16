use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SprucePressurePlate {
    pub powered: bool,
}


impl BlockState for SprucePressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == false { return 6663; }
        if self.r#powered == true { return 6662; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6663 {
            return Some(SprucePressurePlate {
                r#powered: false,
            });
        }
        if state_id == 6662 {
            return Some(SprucePressurePlate {
                r#powered: true,
            });
        }
        return None;
    }
}

