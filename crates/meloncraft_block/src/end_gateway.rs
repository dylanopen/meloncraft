use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndGateway {
}


impl BlockState for EndGateway {
    fn to_id(self) -> i32 {
        return 14614;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14614 {
            return Some(EndGateway {
            });
        }
        return None;
    }
}

