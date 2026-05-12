use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueConcrete {
}


impl BlockState for BlueConcrete {
    fn to_id(self) -> i32 {
        return 14839;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14839 {
            return Some(BlueConcrete {
            });
        }
        return None;
    }
}

