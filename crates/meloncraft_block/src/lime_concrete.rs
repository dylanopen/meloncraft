use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeConcrete {
}


impl BlockState for LimeConcrete {
    fn to_id(&self) -> i32 {
        return 14833;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14833 {
            return Some(LimeConcrete {
            });
        }
        return None;
    }
}

