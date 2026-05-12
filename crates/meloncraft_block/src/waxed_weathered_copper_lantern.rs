use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for WaxedWeatheredCopperLantern {
    fn to_id(self) -> i32 {
        if block_state.r#hanging == false && block_state.r#waterlogged == false { return 20670; }
        if block_state.r#hanging == true && block_state.r#waterlogged == true { return 20667; }
        if block_state.r#hanging == false && block_state.r#waterlogged == true { return 20669; }
        if block_state.r#waterlogged == false && block_state.r#hanging == true { return 20668; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20670 {
            return Some(WaxedWeatheredCopperLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20667 {
            return Some(WaxedWeatheredCopperLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20669 {
            return Some(WaxedWeatheredCopperLantern {
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20668 {
            return Some(WaxedWeatheredCopperLantern {
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        return None;
    }
}

