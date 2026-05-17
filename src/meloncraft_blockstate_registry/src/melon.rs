use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Melon {
}


impl BlockState for Melon {
    fn to_id(&self) -> i32 {
        return 8132;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8132 {
            return Some(Melon {
            });
        }
        return None;
    }
}

