use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackCarpet {
}


impl BlockState for BlackCarpet {
    fn to_id(self) -> i32 {
        return 12709;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12709 {
            return Some(BlackCarpet {
            });
        }
        return None;
    }
}

