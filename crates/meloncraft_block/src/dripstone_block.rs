use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DripstoneBlock {
}


impl BlockState for DripstoneBlock {
    fn to_id(self) -> i32 {
        return 27553;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27553 {
            return Some(DripstoneBlock {
            });
        }
        return None;
    }
}

