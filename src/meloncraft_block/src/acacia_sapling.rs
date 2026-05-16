use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaSapling {
    pub stage: i32,
}


impl BlockState for AcaciaSapling {
    fn to_id(&self) -> i32 {
        if self.r#stage == 1 { return 38; }
        if self.r#stage == 0 { return 37; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 38 {
            return Some(AcaciaSapling {
                r#stage: 1,
            });
        }
        if state_id == 37 {
            return Some(AcaciaSapling {
                r#stage: 0,
            });
        }
        return None;
    }
}

