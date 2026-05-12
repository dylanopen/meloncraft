use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperLantern {
    pub waterlogged: bool,
    pub hanging: bool,
}


impl BlockState for ExposedCopperLantern {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#hanging == true { return 20647; }
        if block_state.r#waterlogged == false && block_state.r#hanging == true { return 20648; }
        if block_state.r#hanging == false && block_state.r#waterlogged == true { return 20649; }
        if block_state.r#waterlogged == false && block_state.r#hanging == false { return 20650; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20647 {
            return Some(ExposedCopperLantern {
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        if state_id == 20648 {
            return Some(ExposedCopperLantern {
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        if state_id == 20649 {
            return Some(ExposedCopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20650 {
            return Some(ExposedCopperLantern {
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        return None;
    }
}

