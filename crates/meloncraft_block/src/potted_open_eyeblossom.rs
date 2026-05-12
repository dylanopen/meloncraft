use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedOpenEyeblossom {
}


impl BlockState for PottedOpenEyeblossom {
    fn to_id(self) -> i32 {
        return 29668;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29668 {
            return Some(PottedOpenEyeblossom {
            });
        }
        return None;
    }
}

