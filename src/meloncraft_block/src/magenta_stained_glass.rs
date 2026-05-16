use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaStainedGlass {
}


impl BlockState for MagentaStainedGlass {
    fn to_id(&self) -> i32 {
        return 6899;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6899 {
            return Some(MagentaStainedGlass {
            });
        }
        return None;
    }
}

