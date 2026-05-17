use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedCrimsonRoots {
}


impl BlockState for PottedCrimsonRoots {
    fn to_id(&self) -> i32 {
        return 21626;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21626 {
            return Some(PottedCrimsonRoots {
            });
        }
        return None;
    }
}

