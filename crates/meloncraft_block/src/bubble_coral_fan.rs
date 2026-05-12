use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleCoralFan {
    pub waterlogged: bool,
}


impl BlockState for BubbleCoralFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true { return 14979; }
        if self.r#waterlogged == false { return 14980; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14979 {
            return Some(BubbleCoralFan {
                r#waterlogged: true,
            });
        }
        if state_id == 14980 {
            return Some(BubbleCoralFan {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

