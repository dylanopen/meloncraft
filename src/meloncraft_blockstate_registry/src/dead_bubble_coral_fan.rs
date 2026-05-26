use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBubbleCoralFan {
    pub waterlogged: bool,
}

impl BlockState for DeadBubbleCoralFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false {
            return 14970;
        }
        if self.r#waterlogged == true {
            return 14969;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14970 {
            return Some(DeadBubbleCoralFan {
                r#waterlogged: false,
            });
        }
        if state_id == 14969 {
            return Some(DeadBubbleCoralFan {
                r#waterlogged: true,
            });
        }
        return None;
    }
}
