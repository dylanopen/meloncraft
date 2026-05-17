use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrackedPolishedBlackstoneBricks {
}


impl BlockState for CrackedPolishedBlackstoneBricks {
    fn to_id(&self) -> i32 {
        return 22042;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22042 {
            return Some(CrackedPolishedBlackstoneBricks {
            });
        }
        return None;
    }
}

