use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBrainCoral {
    pub waterlogged: bool,
}


impl BlockState for DeadBrainCoral {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true { return 14947; }
        if self.r#waterlogged == false { return 14948; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14947 {
            return Some(DeadBrainCoral {
                r#waterlogged: true,
            });
        }
        if state_id == 14948 {
            return Some(DeadBrainCoral {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

