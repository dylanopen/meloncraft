use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutRedSandstone {
}


impl BlockState for CutRedSandstone {
    fn to_id(self) -> i32 {
        return 13047;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13047 {
            return Some(CutRedSandstone {
            });
        }
        return None;
    }
}

