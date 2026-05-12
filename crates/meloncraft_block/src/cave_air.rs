use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaveAir {
}


impl BlockState for CaveAir {
    fn to_id(self) -> i32 {
        return 15091;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15091 {
            return Some(CaveAir {
            });
        }
        return None;
    }
}

