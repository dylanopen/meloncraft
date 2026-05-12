use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfestedStone {
}


impl BlockState for InfestedStone {
    fn to_id(self) -> i32 {
        return 7559;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7559 {
            return Some(InfestedStone {
            });
        }
        return None;
    }
}

