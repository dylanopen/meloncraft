use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleCandleCake {
    pub lit: bool,
}


impl BlockState for PurpleCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false { return 23189; }
        if block_state.r#lit == true { return 23188; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23189 {
            return Some(PurpleCandleCake {
                r#lit: false,
            });
        }
        if state_id == 23188 {
            return Some(PurpleCandleCake {
                r#lit: true,
            });
        }
        return None;
    }
}

