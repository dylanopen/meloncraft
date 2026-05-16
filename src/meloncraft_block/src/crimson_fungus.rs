use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonFungus {
}


impl BlockState for CrimsonFungus {
    fn to_id(&self) -> i32 {
        return 20773;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20773 {
            return Some(CrimsonFungus {
            });
        }
        return None;
    }
}

