use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allium {
}


impl BlockState for Allium {
    fn to_id(&self) -> i32 {
        return 2125;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2125 {
            return Some(Allium {
            });
        }
        return None;
    }
}

