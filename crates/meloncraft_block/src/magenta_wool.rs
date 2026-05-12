use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaWool {
}


impl BlockState for MagentaWool {
    fn to_id(self) -> i32 {
        return 2095;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2095 {
            return Some(MagentaWool {
            });
        }
        return None;
    }
}

