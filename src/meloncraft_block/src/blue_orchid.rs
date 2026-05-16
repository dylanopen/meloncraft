use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueOrchid {
}


impl BlockState for BlueOrchid {
    fn to_id(&self) -> i32 {
        return 2124;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2124 {
            return Some(BlueOrchid {
            });
        }
        return None;
    }
}

