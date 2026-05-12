use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lodestone {
}


impl BlockState for Lodestone {
    fn to_id(self) -> i32 {
        return 21628;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21628 {
            return Some(Lodestone {
            });
        }
        return None;
    }
}

