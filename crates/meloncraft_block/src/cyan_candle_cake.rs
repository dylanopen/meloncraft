use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanCandleCake {
    pub lit: bool,
}


impl BlockState for CyanCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false { return 23187; }
        if block_state.r#lit == true { return 23186; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23187 {
            return Some(CyanCandleCake {
                r#lit: false,
            });
        }
        if state_id == 23186 {
            return Some(CyanCandleCake {
                r#lit: true,
            });
        }
        return None;
    }
}

