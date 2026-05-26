use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedPressurePlate {
    pub powered: bool,
}

impl BlockState for WarpedPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == true {
            return 20846;
        }
        if self.r#powered == false {
            return 20847;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20846 {
            return Some(WarpedPressurePlate { r#powered: true });
        }
        if state_id == 20847 {
            return Some(WarpedPressurePlate { r#powered: false });
        }
        return None;
    }
}
