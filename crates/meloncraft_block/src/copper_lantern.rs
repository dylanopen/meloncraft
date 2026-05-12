use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperLantern {
    pub waterlogged: bool,
    pub hanging: bool,
}


impl BlockState for CopperLantern {
    fn to_id(self) -> i32 {
        if block_state.r#hanging == false && block_state.r#waterlogged == true { return 20645; }
        if block_state.r#hanging == true && block_state.r#waterlogged == true { return 20643; }
        if block_state.r#hanging == true && block_state.r#waterlogged == false { return 20644; }
        if block_state.r#hanging == false && block_state.r#waterlogged == false { return 20646; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20645 {
            return Some(CopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20643 {
            return Some(CopperLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20644 {
            return Some(CopperLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20646 {
            return Some(CopperLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

