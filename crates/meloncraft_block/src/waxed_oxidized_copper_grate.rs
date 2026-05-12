use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperGrate {
    pub waterlogged: bool,
}


impl BlockState for WaxedOxidizedCopperGrate {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 26860; }
        if block_state.r#waterlogged == true { return 26859; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26860 {
            return Some(WaxedOxidizedCopperGrate {
                r#waterlogged: false,
            });
        }
        if state_id == 26859 {
            return Some(WaxedOxidizedCopperGrate {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

