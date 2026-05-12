use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopper {
}


impl BlockState for OxidizedCopper {
    fn to_id(self) -> i32 {
        return 25110;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25110 {
            return Some(OxidizedCopper {
            });
        }
        return None;
    }
}

