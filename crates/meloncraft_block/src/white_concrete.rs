use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteConcrete {
}


impl BlockState for WhiteConcrete {
    fn to_id(self) -> i32 {
        return 14828;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14828 {
            return Some(WhiteConcrete {
            });
        }
        return None;
    }
}

