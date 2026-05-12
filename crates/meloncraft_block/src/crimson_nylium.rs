use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonNylium {
}


impl BlockState for CrimsonNylium {
    fn to_id(self) -> i32 {
        return 20772;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20772 {
            return Some(CrimsonNylium {
            });
        }
        return None;
    }
}

