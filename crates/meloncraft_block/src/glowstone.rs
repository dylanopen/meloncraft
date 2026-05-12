use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glowstone {
}


impl BlockState for Glowstone {
    fn to_id(self) -> i32 {
        return 6815;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6815 {
            return Some(Glowstone {
            });
        }
        return None;
    }
}

