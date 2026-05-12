use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspiciousGravel {
    pub dusted: i32,
}


impl BlockState for SuspiciousGravel {
    fn to_id(self) -> i32 {
        if block_state.r#dusted == 0 { return 125; }
        if block_state.r#dusted == 3 { return 128; }
        if block_state.r#dusted == 1 { return 126; }
        if block_state.r#dusted == 2 { return 127; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 125 {
            return Some(SuspiciousGravel {
                r#dusted: 0,
            });
        }
        if state_id == 128 {
            return Some(SuspiciousGravel {
                r#dusted: 3,
            });
        }
        if state_id == 126 {
            return Some(SuspiciousGravel {
                r#dusted: 1,
            });
        }
        if state_id == 127 {
            return Some(SuspiciousGravel {
                r#dusted: 2,
            });
        }
        return None;
    }
}

