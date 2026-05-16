use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HornCoral {
    pub waterlogged: bool,
}


impl BlockState for HornCoral {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false { return 14964; }
        if self.r#waterlogged == true { return 14963; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14964 {
            return Some(HornCoral {
                r#waterlogged: false,
            });
        }
        if state_id == 14963 {
            return Some(HornCoral {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

