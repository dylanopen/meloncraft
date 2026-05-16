use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueIce {
}


impl BlockState for BlueIce {
    fn to_id(&self) -> i32 {
        return 15073;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15073 {
            return Some(BlueIce {
            });
        }
        return None;
    }
}

