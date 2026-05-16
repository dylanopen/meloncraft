use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diorite {
}


impl BlockState for Diorite {
    fn to_id(&self) -> i32 {
        return 4;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 4 {
            return Some(Diorite {
            });
        }
        return None;
    }
}

