use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneLamp {
    pub lit: bool,
}

impl BlockState for RedstoneLamp {
    fn to_id(&self) -> i32 {
        if self.r#lit == false {
            return 9279;
        }
        if self.r#lit == true {
            return 9278;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9279 {
            return Some(RedstoneLamp { r#lit: false });
        }
        if state_id == 9278 {
            return Some(RedstoneLamp { r#lit: true });
        }
        return None;
    }
}
