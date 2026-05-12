use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperGrate {
    pub waterlogged: bool,
}


impl BlockState for WaxedWeatheredCopperGrate {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true { return 26857; }
        if block_state.r#waterlogged == false { return 26858; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26857 {
            return Some(WaxedWeatheredCopperGrate {
                r#waterlogged: true,
            });
        }
        if state_id == 26858 {
            return Some(WaxedWeatheredCopperGrate {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

