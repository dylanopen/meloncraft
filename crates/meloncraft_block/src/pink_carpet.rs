use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkCarpet {
}


impl BlockState for PinkCarpet {
    fn to_id(&self) -> i32 {
        return 12700;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12700 {
            return Some(PinkCarpet {
            });
        }
        return None;
    }
}

