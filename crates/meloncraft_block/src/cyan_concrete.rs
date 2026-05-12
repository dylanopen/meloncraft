use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanConcrete {
}


impl BlockState for CyanConcrete {
    fn to_id(self) -> i32 {
        return 14837;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14837 {
            return Some(CyanConcrete {
            });
        }
        return None;
    }
}

