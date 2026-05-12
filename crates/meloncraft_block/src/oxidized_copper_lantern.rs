use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for OxidizedCopperLantern {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#hanging == true { return 20655; }
        if block_state.r#hanging == false && block_state.r#waterlogged == true { return 20657; }
        if block_state.r#waterlogged == false && block_state.r#hanging == true { return 20656; }
        if block_state.r#waterlogged == false && block_state.r#hanging == false { return 20658; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20655 {
            return Some(OxidizedCopperLantern {
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        if state_id == 20657 {
            return Some(OxidizedCopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20656 {
            return Some(OxidizedCopperLantern {
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        if state_id == 20658 {
            return Some(OxidizedCopperLantern {
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        return None;
    }
}

