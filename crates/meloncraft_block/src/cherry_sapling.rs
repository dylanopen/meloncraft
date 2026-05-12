use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherrySapling {
    pub stage: i32,
}


impl BlockState for CherrySapling {
    fn to_id(self) -> i32 {
        if block_state.r#stage == 0 { return 39; }
        if block_state.r#stage == 1 { return 40; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 39 {
            return Some(CherrySapling {
                r#stage: 0,
            });
        }
        if state_id == 40 {
            return Some(CherrySapling {
                r#stage: 1,
            });
        }
        return None;
    }
}

