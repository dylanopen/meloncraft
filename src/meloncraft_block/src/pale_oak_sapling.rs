use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakSapling {
    pub stage: i32,
}


impl BlockState for PaleOakSapling {
    fn to_id(&self) -> i32 {
        if self.r#stage == 0 { return 43; }
        if self.r#stage == 1 { return 44; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 43 {
            return Some(PaleOakSapling {
                r#stage: 0,
            });
        }
        if state_id == 44 {
            return Some(PaleOakSapling {
                r#stage: 1,
            });
        }
        return None;
    }
}

