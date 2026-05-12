use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedCarpet {
}


impl BlockState for RedCarpet {
    fn to_id(self) -> i32 {
        return 12708;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12708 {
            return Some(RedCarpet {
            });
        }
        return None;
    }
}

