use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueCandleCake {
    pub lit: bool,
}


impl BlockState for BlueCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false { return 23191; }
        if block_state.r#lit == true { return 23190; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23191 {
            return Some(BlueCandleCake {
                r#lit: false,
            });
        }
        if state_id == 23190 {
            return Some(BlueCandleCake {
                r#lit: true,
            });
        }
        return None;
    }
}

