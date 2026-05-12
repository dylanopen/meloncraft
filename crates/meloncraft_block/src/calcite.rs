use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calcite {
}


impl BlockState for Calcite {
    fn to_id(self) -> i32 {
        return 24485;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24485 {
            return Some(Calcite {
            });
        }
        return None;
    }
}

