use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoidAir {
}


impl BlockState for VoidAir {
    fn to_id(self) -> i32 {
        return 15090;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15090 {
            return Some(VoidAir {
            });
        }
        return None;
    }
}

