use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeCandleCake {
    pub lit: bool,
}


impl BlockState for LimeCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true { return 23178; }
        if block_state.r#lit == false { return 23179; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23178 {
            return Some(LimeCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23179 {
            return Some(LimeCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

