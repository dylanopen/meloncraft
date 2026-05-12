use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackedIce {
}


impl BlockState for PackedIce {
    fn to_id(self) -> i32 {
        return 12712;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12712 {
            return Some(PackedIce {
            });
        }
        return None;
    }
}

