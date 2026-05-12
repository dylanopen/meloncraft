use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonRoots {
}


impl BlockState for CrimsonRoots {
    fn to_id(self) -> i32 {
        return 20829;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20829 {
            return Some(CrimsonRoots {
            });
        }
        return None;
    }
}

