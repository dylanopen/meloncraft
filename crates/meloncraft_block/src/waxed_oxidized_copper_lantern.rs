use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for WaxedOxidizedCopperLantern {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#hanging == true { return 20671; }
        if block_state.r#waterlogged == false && block_state.r#hanging == false { return 20674; }
        if block_state.r#hanging == true && block_state.r#waterlogged == false { return 20672; }
        if block_state.r#hanging == false && block_state.r#waterlogged == true { return 20673; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20671 {
            return Some(WaxedOxidizedCopperLantern {
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        if state_id == 20674 {
            return Some(WaxedOxidizedCopperLantern {
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        if state_id == 20672 {
            return Some(WaxedOxidizedCopperLantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20673 {
            return Some(WaxedOxidizedCopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

