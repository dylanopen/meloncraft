use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaCarpet {
}


impl BlockState for MagentaCarpet {
    fn to_id(self) -> i32 {
        return 12696;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12696 {
            return Some(MagentaCarpet {
            });
        }
        return None;
    }
}

