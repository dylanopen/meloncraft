use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCutCopper {
}


impl BlockState for OxidizedCutCopper {
    fn to_id(&self) -> i32 {
        return 25113;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25113 {
            return Some(OxidizedCutCopper {
            });
        }
        return None;
    }
}

