use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakPressurePlate {
    pub powered: bool,
}

impl BlockState for OakPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == false {
            return 6661;
        }
        if self.r#powered == true {
            return 6660;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6661 {
            return Some(OakPressurePlate { r#powered: false });
        }
        if state_id == 6660 {
            return Some(OakPressurePlate { r#powered: true });
        }
        return None;
    }
}
