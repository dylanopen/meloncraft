use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeaLantern {
}


impl BlockState for SeaLantern {
    fn to_id(self) -> i32 {
        return 12690;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12690 {
            return Some(SeaLantern {
            });
        }
        return None;
    }
}

