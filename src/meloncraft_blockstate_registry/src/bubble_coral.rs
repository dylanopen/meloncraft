use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleCoral {
    pub waterlogged: bool,
}

impl BlockState for BubbleCoral {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 14959;
        }
        if self.r#waterlogged == false {
            return 14960;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14959 {
            return Some(BubbleCoral {
                r#waterlogged: true,
            });
        }
        if state_id == 14960 {
            return Some(BubbleCoral {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
