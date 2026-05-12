use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleColumn {
    pub drag: bool,
}


impl BlockState for BubbleColumn {
    fn to_id(self) -> i32 {
        if block_state.r#drag == true { return 15092; }
        if block_state.r#drag == false { return 15093; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15092 {
            return Some(BubbleColumn {
                r#drag: true,
            });
        }
        if state_id == 15093 {
            return Some(BubbleColumn {
                r#drag: false,
            });
        }
        return None;
    }
}

