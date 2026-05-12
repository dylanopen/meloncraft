use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedOxeyeDaisy {
}


impl BlockState for PottedOxeyeDaisy {
    fn to_id(&self) -> i32 {
        return 10449;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10449 {
            return Some(PottedOxeyeDaisy {
            });
        }
        return None;
    }
}

