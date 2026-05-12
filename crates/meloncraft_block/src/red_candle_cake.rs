use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedCandleCake {
    pub lit: bool,
}


impl BlockState for RedCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true { return 23196; }
        if block_state.r#lit == false { return 23197; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23196 {
            return Some(RedCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23197 {
            return Some(RedCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

