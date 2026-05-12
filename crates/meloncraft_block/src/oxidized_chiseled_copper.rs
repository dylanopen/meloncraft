use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedChiseledCopper {
}


impl BlockState for OxidizedChiseledCopper {
    fn to_id(&self) -> i32 {
        return 25117;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25117 {
            return Some(OxidizedChiseledCopper {
            });
        }
        return None;
    }
}

