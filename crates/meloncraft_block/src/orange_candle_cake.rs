use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeCandleCake {
    pub lit: bool,
}


impl BlockState for OrangeCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true { return 23170; }
        if block_state.r#lit == false { return 23171; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23170 {
            return Some(OrangeCandleCake {
                r#lit: true,
            });
        }
        if state_id == 23171 {
            return Some(OrangeCandleCake {
                r#lit: false,
            });
        }
        return None;
    }
}

