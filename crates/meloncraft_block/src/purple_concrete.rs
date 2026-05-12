use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleConcrete {
}


impl BlockState for PurpleConcrete {
    fn to_id(self) -> i32 {
        return 14838;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14838 {
            return Some(PurpleConcrete {
            });
        }
        return None;
    }
}

