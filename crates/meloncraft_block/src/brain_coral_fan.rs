use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrainCoralFan {
    pub waterlogged: bool,
}


impl BlockState for BrainCoralFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true { return 14977; }
        if self.r#waterlogged == false { return 14978; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14977 {
            return Some(BrainCoralFan {
                r#waterlogged: true,
            });
        }
        if state_id == 14978 {
            return Some(BrainCoralFan {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

