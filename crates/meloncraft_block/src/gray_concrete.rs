use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayConcrete {
}


impl BlockState for GrayConcrete {
    fn to_id(self) -> i32 {
        return 14835;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14835 {
            return Some(GrayConcrete {
            });
        }
        return None;
    }
}

