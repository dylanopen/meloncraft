use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowConcrete {
}


impl BlockState for YellowConcrete {
    fn to_id(&self) -> i32 {
        return 14832;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14832 {
            return Some(YellowConcrete {
            });
        }
        return None;
    }
}

