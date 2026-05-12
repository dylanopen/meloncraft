use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedBamboo {
}


impl BlockState for PottedBamboo {
    fn to_id(self) -> i32 {
        return 15089;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15089 {
            return Some(PottedBamboo {
            });
        }
        return None;
    }
}

