use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchSapling {
    pub stage: i32,
}


impl BlockState for BirchSapling {
    fn to_id(self) -> i32 {
        if block_state.r#stage == 0 { return 33; }
        if block_state.r#stage == 1 { return 34; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 33 {
            return Some(BirchSapling {
                r#stage: 0,
            });
        }
        if state_id == 34 {
            return Some(BirchSapling {
                r#stage: 1,
            });
        }
        return None;
    }
}

