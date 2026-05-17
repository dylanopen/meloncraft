use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terracotta {
}


impl BlockState for Terracotta {
    fn to_id(&self) -> i32 {
        return 12710;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12710 {
            return Some(Terracotta {
            });
        }
        return None;
    }
}

