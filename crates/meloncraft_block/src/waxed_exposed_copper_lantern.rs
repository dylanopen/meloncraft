use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for WaxedExposedCopperLantern {
    fn to_id(self) -> i32 {
        if block_state.r#hanging == false && block_state.r#waterlogged == true { return 20665; }
        if block_state.r#waterlogged == true && block_state.r#hanging == true { return 20663; }
        if block_state.r#hanging == true && block_state.r#waterlogged == false { return 20664; }
        if block_state.r#hanging == false && block_state.r#waterlogged == false { return 20666; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20665 {
            return Some(WaxedExposedCopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20663 {
            return Some(WaxedExposedCopperLantern {
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        if state_id == 20664 {
            return Some(WaxedExposedCopperLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20666 {
            return Some(WaxedExposedCopperLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

