use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SculkCatalyst {
    pub bloom: bool,
}


impl BlockState for SculkCatalyst {
    fn to_id(self) -> i32 {
        if block_state.r#bloom == false { return 25098; }
        if block_state.r#bloom == true { return 25097; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25098 {
            return Some(SculkCatalyst {
                r#bloom: false,
            });
        }
        if state_id == 25097 {
            return Some(SculkCatalyst {
                r#bloom: true,
            });
        }
        return None;
    }
}

