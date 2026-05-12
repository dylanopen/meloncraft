use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherGoldOre {
}


impl BlockState for NetherGoldOre {
    fn to_id(self) -> i32 {
        return 135;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 135 {
            return Some(NetherGoldOre {
            });
        }
        return None;
    }
}

