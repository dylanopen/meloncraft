use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakPressurePlate {
    pub powered: bool,
}

impl BlockState for DarkOakPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#powered == true {
            return 6672;
        }
        if self.r#powered == false {
            return 6673;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6672 {
            return Some(DarkOakPressurePlate { r#powered: true });
        }
        if state_id == 6673 {
            return Some(DarkOakPressurePlate { r#powered: false });
        }
        return None;
    }
}
