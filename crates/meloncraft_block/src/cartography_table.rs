use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartographyTable {
}


impl BlockState for CartographyTable {
    fn to_id(self) -> i32 {
        return 20568;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20568 {
            return Some(CartographyTable {
            });
        }
        return None;
    }
}

