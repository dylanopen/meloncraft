use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoalBlock {
}


impl BlockState for CoalBlock {
    fn to_id(self) -> i32 {
        return 12711;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12711 {
            return Some(CoalBlock {
            });
        }
        return None;
    }
}

