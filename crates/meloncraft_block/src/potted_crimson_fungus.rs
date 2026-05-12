use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedCrimsonFungus {
}


impl BlockState for PottedCrimsonFungus {
    fn to_id(self) -> i32 {
        return 21624;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21624 {
            return Some(PottedCrimsonFungus {
            });
        }
        return None;
    }
}

