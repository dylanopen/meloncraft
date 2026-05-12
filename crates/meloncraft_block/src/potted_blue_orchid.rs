use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedBlueOrchid {
}


impl BlockState for PottedBlueOrchid {
    fn to_id(&self) -> i32 {
        return 10442;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10442 {
            return Some(PottedBlueOrchid {
            });
        }
        return None;
    }
}

