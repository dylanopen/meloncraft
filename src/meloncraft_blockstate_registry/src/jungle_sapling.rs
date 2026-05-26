use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleSapling {
    pub stage: i32,
}

impl BlockState for JungleSapling {
    fn to_id(&self) -> i32 {
        if self.r#stage == 0 {
            return 35;
        }
        if self.r#stage == 1 {
            return 36;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 35 {
            return Some(JungleSapling { r#stage: 0 });
        }
        if state_id == 36 {
            return Some(JungleSapling { r#stage: 1 });
        }
        return None;
    }
}
