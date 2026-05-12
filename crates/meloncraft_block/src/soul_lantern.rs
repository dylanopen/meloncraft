use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for SoulLantern {
    fn to_id(self) -> i32 {
        if block_state.r#hanging == true && block_state.r#waterlogged == false { return 20640; }
        if block_state.r#hanging == true && block_state.r#waterlogged == true { return 20639; }
        if block_state.r#waterlogged == false && block_state.r#hanging == false { return 20642; }
        if block_state.r#waterlogged == true && block_state.r#hanging == false { return 20641; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20640 {
            return Some(SoulLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20639 {
            return Some(SoulLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20642 {
            return Some(SoulLantern {
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        if state_id == 20641 {
            return Some(SoulLantern {
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        return None;
    }
}

