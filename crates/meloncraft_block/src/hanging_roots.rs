use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HangingRoots {
    pub waterlogged: bool,
}


impl BlockState for HangingRoots {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true { return 27717; }
        if self.r#waterlogged == false { return 27718; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27717 {
            return Some(HangingRoots {
                r#waterlogged: true,
            });
        }
        if state_id == 27718 {
            return Some(HangingRoots {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

