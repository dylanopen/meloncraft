use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueCarpet {
}


impl BlockState for BlueCarpet {
    fn to_id(self) -> i32 {
        return 12705;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12705 {
            return Some(BlueCarpet {
            });
        }
        return None;
    }
}

