use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FireCoralFan {
    pub waterlogged: bool,
}


impl BlockState for FireCoralFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false { return 14982; }
        if self.r#waterlogged == true { return 14981; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14982 {
            return Some(FireCoralFan {
                r#waterlogged: false,
            });
        }
        if state_id == 14981 {
            return Some(FireCoralFan {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

