use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeFern {
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for LargeFern {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Upper { return 12723; }
        if block_state.r#half == Half::Lower { return 12724; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12723 {
            return Some(LargeFern {
                r#half: Half::Upper,
            });
        }
        if state_id == 12724 {
            return Some(LargeFern {
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

