use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakSapling {
    pub stage: i32,
}


impl BlockState for OakSapling {
    fn to_id(self) -> i32 {
        if block_state.r#stage == 1 { return 30; }
        if block_state.r#stage == 0 { return 29; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 30 {
            return Some(OakSapling {
                r#stage: 1,
            });
        }
        if state_id == 29 {
            return Some(OakSapling {
                r#stage: 0,
            });
        }
        return None;
    }
}

