use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FletchingTable {
}


impl BlockState for FletchingTable {
    fn to_id(&self) -> i32 {
        return 20569;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20569 {
            return Some(FletchingTable {
            });
        }
        return None;
    }
}

