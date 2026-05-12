use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadFireCoralFan {
    pub waterlogged: bool,
}


impl BlockState for DeadFireCoralFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false { return 14972; }
        if self.r#waterlogged == true { return 14971; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14972 {
            return Some(DeadFireCoralFan {
                r#waterlogged: false,
            });
        }
        if state_id == 14971 {
            return Some(DeadFireCoralFan {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

