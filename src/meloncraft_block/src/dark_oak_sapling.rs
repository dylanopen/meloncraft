use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakSapling {
    pub stage: i32,
}


impl BlockState for DarkOakSapling {
    fn to_id(&self) -> i32 {
        if self.r#stage == 0 { return 41; }
        if self.r#stage == 1 { return 42; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 41 {
            return Some(DarkOakSapling {
                r#stage: 0,
            });
        }
        if state_id == 42 {
            return Some(DarkOakSapling {
                r#stage: 1,
            });
        }
        return None;
    }
}

