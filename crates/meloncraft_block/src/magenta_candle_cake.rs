use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaCandleCake {
    pub lit: bool,
}


impl BlockState for MagentaCandleCake {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false { return 23173; }
        if block_state.r#lit == true { return 23172; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23173 {
            return Some(MagentaCandleCake {
                r#lit: false,
            });
        }
        if state_id == 23172 {
            return Some(MagentaCandleCake {
                r#lit: true,
            });
        }
        return None;
    }
}

