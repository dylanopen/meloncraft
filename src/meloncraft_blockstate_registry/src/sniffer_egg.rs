use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnifferEgg {
    pub hatch: i32,
}

impl BlockState for SnifferEgg {
    fn to_id(&self) -> i32 {
        if self.r#hatch == 0 {
            return 14900;
        }
        if self.r#hatch == 1 {
            return 14901;
        }
        if self.r#hatch == 2 {
            return 14902;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14900 {
            return Some(SnifferEgg { r#hatch: 0 });
        }
        if state_id == 14901 {
            return Some(SnifferEgg { r#hatch: 1 });
        }
        if state_id == 14902 {
            return Some(SnifferEgg { r#hatch: 2 });
        }
        return None;
    }
}
