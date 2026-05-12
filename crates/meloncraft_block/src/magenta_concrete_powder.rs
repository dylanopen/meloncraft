use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaConcretePowder {
}


impl BlockState for MagentaConcretePowder {
    fn to_id(&self) -> i32 {
        return 14846;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14846 {
            return Some(MagentaConcretePowder {
            });
        }
        return None;
    }
}

