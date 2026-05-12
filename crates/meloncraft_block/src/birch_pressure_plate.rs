use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchPressurePlate {
    pub powered: bool,
}


impl BlockState for BirchPressurePlate {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false { return 6665; }
        if block_state.r#powered == true { return 6664; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6665 {
            return Some(BirchPressurePlate {
                r#powered: false,
            });
        }
        if state_id == 6664 {
            return Some(BirchPressurePlate {
                r#powered: true,
            });
        }
        return None;
    }
}

