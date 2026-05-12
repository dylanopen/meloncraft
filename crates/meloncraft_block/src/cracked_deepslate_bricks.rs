use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrackedDeepslateBricks {
}


impl BlockState for CrackedDeepslateBricks {
    fn to_id(self) -> i32 {
        return 29369;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29369 {
            return Some(CrackedDeepslateBricks {
            });
        }
        return None;
    }
}

